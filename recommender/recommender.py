import pandas as pd
import numpy as np
import torch
import torch.nn.functional as F
import compress_fasttext


def get_recommendations(worker, tasks):
    worker = pd.DataFrame(worker, columns=['complexity', 'time', 'tags'])
    tasks = pd.DataFrame(tasks, columns=['complexity', 'time', 'tags'])
    model = compress_fasttext.models.CompressedFastTextKeyedVectors.load('quantized_fasttext_model.bin')

    def vectorize_tags(tag_rows):
        vectorized_tags = []
        for tag_row in tag_rows:
            arrays = []
            for tag in tag_row:
                arrays.append(model[tag])
            if arrays:
                vectorized_tags.append(np.mean(np.array(arrays), axis=0))
            else:
                vectorized_tags.append(np.zeros(model.vector_size))
        return vectorized_tags

    worker['vec_tags'] = vectorize_tags(worker['tags'])
    tasks['vec_tags'] = vectorize_tags(tasks['tags'])

    tag_weight = 30
    similarity_threshold = 0.45
    sim = {}
    worker = worker.iloc[0]
    worker_tags_vector = torch.tensor((worker['vec_tags'] * tag_weight), dtype=torch.float32)
    worker_oth_vector = torch.tensor([worker['complexity'], worker['time']], dtype=torch.float32)
    for task_index, task in tasks.iterrows():
        task_tags_vector = torch.tensor(task['vec_tags'] * tag_weight, dtype=torch.float32)
        tag_sim = F.cosine_similarity(worker_tags_vector, task_tags_vector, dim=0)
        if tag_sim.item() >= similarity_threshold:
            sim[task_index] = [tag_sim.item()]

    for task_index in sim:
        task_oth_vector = torch.tensor([tasks.iloc[task_index]['complexity'], tasks.iloc[task_index]['time']],
                                       dtype=torch.float32)
        oth_sim = F.cosine_similarity(worker_oth_vector, task_oth_vector, dim=0)
        sim[task_index].append(oth_sim.item())
    sorted_tasks = []
    for task_index in sim:
        if not sorted_tasks:
            sorted_tasks.append(task_index)
        else:
            for i, st_task_index in enumerate(sorted_tasks):
                if sim[task_index][0] > sim[st_task_index][0]:
                    sorted_tasks.insert(i, task_index)
                    break
                elif sim[task_index][0] == sim[st_task_index][0]:
                    if sim[task_index][1] > sim[st_task_index][1]:
                        sorted_tasks.insert(i, task_index)
                        break
                    elif i == len(sorted_tasks) - 1:
                        sorted_tasks.append(task_index)
                        break
                elif i == len(sorted_tasks) - 1:
                    sorted_tasks.append(task_index)
                    break
    return [tasks.iloc[x].to_list()[:-1] for x in sorted_tasks]

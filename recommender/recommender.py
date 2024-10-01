import numpy as np
import compress_fasttext


def get_recommendations(worker, tasks):
    model = compress_fasttext.models.CompressedFastTextKeyedVectors.load('quantized_large_model.bin')

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

    worker_vec_tags = vectorize_tags([worker[2]])[0]

    tasks_vec_tags = vectorize_tags([task[2] for task in tasks])

    tag_weight = 30
    similarity_threshold = 0.45
    sim = {}

    worker_tags_vector = np.array(worker_vec_tags) * tag_weight
    worker_oth_vector = np.array([worker[0], worker[1]])

    for task_index, task in enumerate(tasks):
        task_tags_vector = np.array(tasks_vec_tags[task_index]) * tag_weight
        tag_sim = np.dot(worker_tags_vector, task_tags_vector) / (
                    np.linalg.norm(worker_tags_vector) * np.linalg.norm(task_tags_vector))

        if tag_sim >= similarity_threshold:
            sim[task_index] = [tag_sim]

    for task_index in sim:
        task_oth_vector = np.array([tasks[task_index][0], tasks[task_index][1]])
        oth_sim = np.dot(worker_oth_vector, task_oth_vector) / (
                    np.linalg.norm(worker_oth_vector) * np.linalg.norm(task_oth_vector))
        sim[task_index].append(oth_sim)

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
    return [tasks[x] for x in sorted_tasks]


if __name__ == '__main__':
    # Preview
    tasks_list = [
        [2, 11, ["Docker", "Kubernetes", "CI/CD"]],
        [1, 3.1, ["Docker", "Kubernetes"]],
        [2, 16, ["Kubernetes"]],
        [0, 11, ["Docker", "Kubernetes", "CI/CD"]],
        [1.5, 11, ["UI/UX Design", "Design"]],
        [1.5, 11, ["Backend"]],
        [3, 8, ["React", "JavaScript", "CSS"]],
        [2, 5, ["Vue.js", "JavaScript"]],
        [1, 7, ["HTML", "CSS", "JavaScript"]],
        [2.5, 9, ["Angular", "TypeScript"]],
        [1, 4, ["JavaScript", "CSS"]],
        [3, 10, ["React", "Redux"]],
        [2, 10, ["Python", "Machine Learning", "Pandas"]],
        [3, 12, ["R", "Data Analysis"]],
        [1.5, 8, ["SQL", "Data Visualization"]],
        [2, 15, ["Python", "TensorFlow"]],
        [2.5, 10, ["Statistics", "Data Mining"]],
        [1, 5, ["Python", "Numpy"]],
        [1.5, 6, ["Linux", "Shell Scripting"]],
        [2, 8, ["Windows Server", "Active Directory"]],
        [2.5, 10, ["Network Configuration", "Firewall"]],
        [1, 4, ["Backup Solutions", "Data Recovery"]],
        [3, 12, ["Cloud Services", "AWS"]],
        [2, 7, ["Linux", "Docker"]],
        [2, 9, ["Swift", "iOS Development"]],
        [1.5, 7, ["Kotlin", "Android Development"]],
        [2, 8, ["React Native", "JavaScript"]],
        [3, 10, ["Flutter", "Dart"]],
        [1, 5, ["Objective-C", "iOS"]],
        [2.5, 11, ["Java", "Android"]]
    ]
    workers = [
        [1.5, 11, ["devops", "Docker", "Kubernetes", "CI/CD"]],
        [2, 7, ["JavaScript", "React", "CSS", "HTML"]],
        [2.5, 12, ["Python", "Machine Learning", "Data Analysis", "SQL"]],
        [2, 9, ["Linux", "Network Configuration", "Cloud Services", "Shell Scripting"]],
        [2, 8, ["Kotlin", "Android Development", "Java", "React Native"]]
    ]
    for worker in workers:
        recommendations = get_recommendations(worker, tasks_list)
        print('++++++++++++++++++++++worker++++++++++++++++++++++++++')
        print(worker)
        for i, task in enumerate(recommendations):
            print(f'=====================task top-{i+1}=======================')
            print(task)
            print('======================================================')
        print('++++++++++++++++++++++++++++++++++++++++++++++++++++++')
        print()

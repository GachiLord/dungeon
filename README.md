# Dark Dangeon
Dark Dangeon - это таск-трекер для небольших IT-команд, работающих по методологиям, вдохновленными Agile Manifesto. Приложение стилизовано под визуальную новеллу в D&D-подобном сеттинге, пользователь является авантюристом, который выполняет опасные задания ради славы и места на доске почета.

## Информация, связанная с хакатоном
1. Название задачи: **Геймификация взаимодействия в iT-командах**
1. Состав команды: 
    * **Николай Прокофьев**
    * **Иван Черненко**

## Описание репозитория

  * **server** - http-сервер, который занимается обработкой запросов от пользователей. Сервер написан на rust(axum, tokio_postgres, tera), клиент - html, css(rpgui), js(htmx)
  * **db** - хранение базы данных(Postgres) и всего, что с ней связано
  * **recommender** - http-сервер с ai-моделью, выдающей рекомендации, написан на python(ompress-fasttext, gensim, scipy, smart-open, wrapt, scikit-learn)

## Развертывание

Для развертывания вам понадобиться:
- docker, docker compose
- GPU(скорее всего)
- Открытые порты 80 и 5432

1. Переходим в папку с проектом

2. Запускаем сервер этой командой:
```bash
docker compose up -d
```
3. Если происходят ошибки связанные с сервисом dungeon-recommender, попробуйте запустить проект этой командой:
```bash
docker compose up db web -d
```

4. После успешного запуска, необходимо создать привелегированного пользователя(для отправки инвайтов), выполнив команду:
```bash

docker compose exec db psql -U dungeon -W -c "insert into users (login, name, password, class, is_admin, tags) values('text', 'text', '\$argon2i\$v=19\$m=32,t=3,p=4\$c2FsdHNhbHQ\$N5OSJjxpM+8ueBlykYlg/cGn8Nx8jMmGRew76u5w', 0, true, '{}')"
```
Эта команда создаст пользователя с логином text и паролем text. Если нужен другой пароль, сгенерируйте argon2-hash и вставьте вместо "$argon2i$v=19$m=32,t=3,p=4$c2FsdHNhbHQ$N5OSJjxpM+8ueBlykYlg/cGn8Nx8jMmGRew76u5w".

5. Открыть в браузере http://localhost

## Советы
* Чтобы персонаж произнес новую реплику, нажмите по диалоговому окну
* Чтобы пригласить пользователя, перейдите в "Убежище" и нажмите кнопку "Скопировать приглашение", залогинившись под привелегированным пользователем.

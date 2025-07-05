POST http://127.0.0.1:3000/simpletest/v1/article/new
[Form]
title: test111
content: this is the content of test111
authorname: mike tang
HTTP 200
[Captures]
id: jsonpath "$[0].id"

GET http://127.0.0.1:3000/simpletest/v1/article?id={{id}}
HTTP 200

PUT http://127.0.0.1:3000/simpletest/v1/article/update
[Form]
id: {{id}}
title: test_updated
content: updated, this is the content of test111, updated.
authorname: mike tang2
HTTP 200

DELETE http://127.0.0.1:3000/simpletest/v1/article/delete
[Form]
id: {{id}}
HTTP 200

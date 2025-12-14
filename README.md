## simple api - rust
this is a simple api with rust programming language

### how to run it at youre environtment
-   install depencies
```bash
cargo build
```

-   run
```bash
cargo run
```

call the API from bash
-   Create a new Student
```bash
curl -X POST http://127.0.0.1:8080/api/v1/students \
-H "Content-Type: application/json" \
-d '{
    "name": "Mr X", 
    "major": "Information Technology", 
    "enrollment_year": 2024
}'
```

-   Create a new Lesson
```bash
curl -X POST http://127.0.0.1:8080/api/v1/lessons \
-H "Content-Type: application/json" \
-d '{
    "name": "Object-Oriented Programming", 
    "credits": 4, 
    "teacher": "Prof. xy"
}'
```

-   Create a new Grade
```bash
curl -X POST http://127.0.0.1:8080/api/v1/grades \
-H "Content-Type: application/json" \
-d '{
    "student_id": "[student_id]", 
    "lesson_id": "[lesson_id]", 
    "score": 95.5,
    "semester": "Spring 2025"
}'
```
example
```bash
curl -X POST http://127.0.0.1:8080/api/v1/grades \
-H "Content-Type: application/json" \
-d '{
    "student_id": "1", 
    "lesson_id": "1", 
    "score": 95.5,
    "semester": "Winter 2025"
}'
```
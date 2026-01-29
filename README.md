# Diplomind Backend

Backend REST API for the Diplomind skill management platform. Built with Rust using the Poem framework.

## Description

Diplomind is an educational platform designed to manage student skill acquisition and validation. The system allows:

- Administrators to manage users, classes, and courses
- Teachers to create projects, define required skills, and validate student skills
- Students to track their skill progression and validation status

## Technology Stack

- **Language**: Rust
- **Framework**: Poem
- **Database**: PostgreSQL hosted in a Docker VM
- **SQL query handler**: SQLx
- **Authentication**:
  - Access token : Some user informations stored in a JsonWebToken of short validity in the localstorage.
  - Refresh token : Base64-encoded string representing 32 bytes (256 bits) of cryptographically random data stored in the cookie defined in the ".env" file and in the database.
- **Password Hashing Algorithm**: Argon2

## Getting Started

### Prerequisites

- Rust 1.70+
- Docker
- Just (command runner) #Non mandatory but helps for quick and easy commands. Even without <b><i>Just</i></b> you can just copy/paste the commands from the file.

### Installation

#### Clone the repository

```bash
git clone https://github.com/Mzgus/diplomind_be.git
cd diplomind_be
```

#### Configure environment

```bash
cp .env.template .env
```

Edit .env with all your informations

#### Initialize database

```bash
just reboot
```

or

```bash
docker compose down 
docker volume rm diplomind
docker compose up -d
```

#### Run the server

```bash
cargo run
```

The API will be available at `http://localhost:3000`.

---

## Role-Based Access Control

The API implements three user roles with distinct permissions:

| Role        | Permissions                                                      |
| ----------- | ---------------------------------------------------------------- |
| **admin**   | Full access to all resources, user management, security controls |
| **teacher** | Manage courses, projects, steps, skills; validate student skills |
| **student** | View own profile, courses, and skill validations (read-only)     |

---

## API Routes

All protected routes require a valid JWT token in the `Authorization: Bearer <token>` header.

### Authentication

**User Story**: As a user, I need to authenticate to access the platform and maintain a secure session.

| Method | Route             | Description                   | Auth         |
| ------ | ----------------- | ----------------------------- | ------------ |
| POST   | `/login`          | User authentication           | None         |
| GET    | `/refresh_tokens` | Refresh token pair            | Cookie based |
| GET    | `/logout`         | Invalidate session            | Student      |
| GET    | `/verify_token`   | Validate current access token | Student      |

---

### Users

**User Story**: As an administrator, I need to manage user accounts to maintain the platform user base.

| Method | Route                      | Description                | Auth          |
| ------ | -------------------------- | -------------------------- | ------------- |
| GET    | `/users`                   | List all users             | Student       |
| GET    | `/users/:id`               | Get user by ID             | Student       |
| GET    | `/users/email/:email`      | Get user by email          | Student       |
| GET    | `/users_sheets`            | List user profiles         | Admin         |
| POST   | `/users_sheets`            | Create user profile        | Admin         |
| GET    | `/users_sheets/:id`        | Get user profile by ID     | Admin or Self |
| PUT    | `/users_sheets/:id`        | Update user profile        | Admin         |
| DELETE | `/users_sheets/:id`        | Delete user profile        | Admin         |
| POST   | `/users_auth`              | Create user credentials    | Admin         |
| GET    | `/users_auth/:id`          | Get user credentials by ID | Admin or Self |
| DELETE | `/users_auth/:id`          | Delete user credentials    | Admin         |
| PATCH  | `/users_auth/:id/email`    | Update email               | Admin or Self |
| PATCH  | `/users_auth/:id/password` | Update password            | Admin or Self |

---

### Administration

**User Story**: As an administrator, I need security controls to manage user access and respond to security incidents.

| Method | Route                                 | Description                | Auth  |
| ------ | ------------------------------------- | -------------------------- | ----- |
| PATCH  | `/admin/users/:id/deactivate`         | Deactivate user account    | Admin |
| PATCH  | `/admin/users/:id/activate`           | Activate user account      | Admin |
| POST   | `/admin/security/revoke-all-sessions` | Revoke all active sessions | Admin |

---

### Classes

**User Story**: As an administrator, I need to organize students into classes for course assignment and tracking.

| Method | Route                   | Description             | Auth                    |
| ------ | ----------------------- | ----------------------- | ----------------------- |
| GET    | `/classes`              | List all classes        | Teacher                 |
| POST   | `/classes`              | Create class            | Admin                   |
| GET    | `/classes/:id`          | Get class details by ID | Teacher                 |
| PUT    | `/classes/:id`          | Update class            | Admin                   |
| DELETE | `/classes/:id`          | Delete class            | Admin                   |
| GET    | `/teachers/:id/classes` | Get teacher's classes   | Teacher (Self) or Admin |

---

### Skills

**User Story**: As a teacher, I need to define skills that students will acquire through projects and coursework.

| Method | Route         | Description             | Auth    |
| ------ | ------------- | ----------------------- | ------- |
| GET    | `/skills`     | List all skills         | Student |
| POST   | `/skills`     | Create skill            | Teacher |
| GET    | `/skills/:id` | Get skill details by ID | Student |
| PUT    | `/skills/:id` | Update skill            | Teacher |
| DELETE | `/skills/:id` | Delete skill            | Teacher |

---

### Courses

**User Story**: As an administrator, I need to create and manage courses that group related projects and skills.

| Method | Route          | Description              | Auth    |
| ------ | -------------- | ------------------------ | ------- |
| GET    | `/courses`     | List all courses         | Student |
| POST   | `/courses`     | Create course            | Admin   |
| GET    | `/courses/:id` | Get course details by ID | Student |
| PUT    | `/courses/:id` | Update course            | Admin   |
| DELETE | `/courses/:id` | Delete course            | Admin   |

---

### Projects

**User Story**: As a teacher, I need to create and manage projects within courses that allow students to demonstrate skills.

| Method | Route                   | Description               | Auth                      |
| ------ | ----------------------- | ------------------------- | ------------------------- |
| GET    | `/projects`             | List all projects         | Student                   |
| POST   | `/projects`             | Create project            | Teacher                   |
| GET    | `/projects/:id`         | Get project details by ID | Student                   |
| PUT    | `/projects/:id`         | Update project            | Teacher                   |
| DELETE | `/projects/:id`         | Delete project            | Teacher                   |
| GET    | `/courses/:id/projects` | List projects by course   | Student                   |
| GET    | `/users/:id/projects`   | Get student's projects    | Student (Self) or Teacher |

---

### Steps

**User Story**: As a teacher, I need to break down projects into steps to guide students through skill acquisition.

| Method | Route                 | Description            | Auth    |
| ------ | --------------------- | ---------------------- | ------- |
| GET    | `/steps`              | List all steps         | Student |
| POST   | `/steps`              | Create step            | Teacher |
| GET    | `/steps/:id`          | Get step details by ID | Student |
| PUT    | `/steps/:id`          | Update step            | Teacher |
| DELETE | `/steps/:id`          | Delete step            | Teacher |
| GET    | `/projects/:id/steps` | List steps by project  | Student |

---

### User-Class Assignments

**User Story**: As an administrator, I need to enroll students in classes to manage their course access.

| Method | Route                               | Description            | Auth    |
| ------ | ----------------------------------- | ---------------------- | ------- |
| POST   | `/user-classes`                     | Assign user to class   | Admin   |
| GET    | `/users/:id/classes`                | Get user's classes     | Student |
| GET    | `/classes/:id/users`                | Get class members      | Teacher |
| DELETE | `/users/:user_id/classes/:class_id` | Remove user from class | Admin   |

---

### User-Course Enrollments

**User Story**: As an administrator, I need to manage course enrollments to control student access to learning materials.

| Method | Route                                | Description             | Auth    |
| ------ | ------------------------------------ | ----------------------- | ------- |
| POST   | `/user-courses`                      | Enroll user in course   | Admin   |
| GET    | `/users/:id/courses`                 | Get user's courses      | Student |
| GET    | `/courses/:id/users`                 | Get course participants | Teacher |
| DELETE | `/users/:user_id/courses/:course_id` | Remove user from course | Admin   |

---

### Course-Class Associations

**User Story**: As an administrator, I need to assign courses to specific classes to manage curriculum delivery.

| Method | Route                                   | Description              | Auth    |
| ------ | --------------------------------------- | ------------------------ | ------- |
| POST   | `/course-classes`                       | Link course to class     | Admin   |
| GET    | `/courses/:id/classes`                  | Get course's classes     | Teacher |
| GET    | `/classes/:id/courses`                  | Get class's courses      | Teacher |
| DELETE | `/courses/:course_id/classes/:class_id` | Unlink course from class | Admin   |

---

### Course-Skill Associations

**User Story**: As a teacher, I need to define which skills are covered by each course for curriculum planning.

| Method | Route                                  | Description              | Auth    |
| ------ | -------------------------------------- | ------------------------ | ------- |
| POST   | `/course-skills`                       | Link skill to course     | Teacher |
| GET    | `/courses/:id/skills`                  | Get course's skills      | Student |
| GET    | `/skills/:id/courses`                  | Get skill's courses      | Student |
| DELETE | `/courses/:course_id/skills/:skill_id` | Unlink skill from course | Teacher |

---

### Step-Skill Associations

**User Story**: As a teacher, I need to map skills to project steps to track which skills are demonstrated at each stage.

| Method | Route                              | Description            | Auth    |
| ------ | ---------------------------------- | ---------------------- | ------- |
| POST   | `/step-skills`                     | Link skill to step     | Teacher |
| GET    | `/steps/:id/skills`                | Get step's skills      | Student |
| GET    | `/skills/:id/steps`                | Get skill's steps      | Student |
| DELETE | `/steps/:step_id/skills/:skill_id` | Unlink skill from step | Teacher |

---

### Skill Validations

**User Story**: As a teacher, I need to validate student skills and provide feedback to track their progression.

| Method | Route                                            | Description                        | Auth                      |
| ------ | ------------------------------------------------ | ---------------------------------- | ------------------------- |
| POST   | `/skill-validations`                             | Create validation record           | Teacher                   |
| GET    | `/skill-validations/user/:user_id`               | Get user's validations             | Student                   |
| GET    | `/skill-validations/pending`                     | List pending validations           | Teacher                   |
| GET    | `/skill-validations/:user_id/:skill_id`          | Get validation details             | Student                   |
| PATCH  | `/skill-validations/:user_id/:skill_id`          | Update validation status           | Teacher                   |
| DELETE | `/skill-validations/:user_id/:skill_id`          | Delete validation                  | Admin                     |
| GET    | `/users/:user_id/courses/:course_id/validations` | Get student validations for course | Student (Self) or Teacher |

---

---

## Data Transfer Objects

### Authentication

```rust
LoginRequest {
    email: String,
    pwd: String
}

AuthTokenResponse {
    token: String,
    token_type: String
}
```

### Users

```rust
CreateUserSheet {
    nom: String,
    prenom: String,
    type_user: String,
    avatar: Option<String>
}

UpdateUserSheet {
    nom: Option<String>,
    prenom: Option<String>,
    type_user: Option<String>,
    avatar: Option<String>
}

CreateUserAuth {
    email: String,
    pwd: String,
    id_user_sheet: i32
}
```

### Classes

```rust
CreateClass {
    name: String,
    description: Option<String>,
    year: i32
}

UpdateClass {
    name: Option<String>,
    description: Option<String>,
    year: Option<i32>
}
```

### Skills

```rust
CreateSkill {
    name: String,
    description: Option<String>
}

UpdateSkill {
    name: Option<String>,
    description: Option<String>
}
```

### Courses

```rust
CreateCourse {
    name: String,
    description: Option<String>,
    year: Option<i32>
}

UpdateCourse {
    name: Option<String>,
    description: Option<String>,
    year: Option<i32>
}
```

### Projects

```rust
CreateProject {
    name: String,
    description: Option<String>,
    course_id: i32
}

UpdateProject {
    name: Option<String>,
    description: Option<String>,
    course_id: Option<i32>
}
```

### Steps

```rust
CreateStep {
    name: String,
    description: Option<String>,
    order_index: i32,
    project_id: i32
}

UpdateStep {
    name: Option<String>,
    description: Option<String>,
    order_index: Option<i32>
}
```

### Skill Validations

```rust
CreateValidationRequest {
    user_id: i32,
    skill_id: i32,
    status: String,
    comment: Option<String>
}

UpdateValidationStatus {
    status: String,
    comment: Option<String>
}
```

---

## Testing

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test <test_name>

# Run with output
cargo test -- --nocapture
```

# Diplomind Backend

Backend REST API for the Diplomind skill management platform. Built with Rust using the Poem framework.

## Description

Diplomind is an educational platform designed to manage student skill acquisition and validation. The system allows:

- Administrators to manage users, classes, and courses
- Teachers to create projects, define required skills, and validate student competencies
- Students to track their skill progression and validation status

## Technology Stack

- **Language**: Rust
- **Framework**: Poem
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Authentication**: JWT (jsonwebtoken)
- **Password Hashing**: Argon2

## Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Just (command runner)

### Installation

```bash
# Clone the repository
git clone https://github.com/Mzgus/diplomind_be.git
cd diplomind_be

# Configure environment
cp .env.example .env
# Edit .env with your database credentials

# Initialize database
just reboot

# Run the server
cargo run
```

The API will be available at `http://localhost:3000`.

---

## API Routes

All protected routes require a valid JWT token in the `Authorization: Bearer <token>` header.

### Authentication

| Method | Route             | Description            | Auth |
| ------ | ----------------- | ---------------------- | ---- |
| POST   | `/login`          | User authentication    | No   |
| GET    | `/refresh_tokens` | Refresh access token   | No   |
| GET    | `/logout`         | Invalidate session     | Yes  |
| GET    | `/verify_token`   | Validate current token | Yes  |

**User Story**: As a user, I need to authenticate to access the platform and maintain a secure session.

---

### Users

| Method | Route                      | Description             | Auth |
| ------ | -------------------------- | ----------------------- | ---- |
| GET    | `/users`                   | List all users          | Yes  |
| GET    | `/users/:id`               | Get user by ID          | Yes  |
| GET    | `/users/email/:email`      | Get user by email       | Yes  |
| GET    | `/users_sheets`            | List user profiles      | Yes  |
| POST   | `/users_sheets`            | Create user profile     | Yes  |
| GET    | `/users_sheets/:id`        | Get user profile        | Yes  |
| PUT    | `/users_sheets/:id`        | Update user profile     | Yes  |
| DELETE | `/users_sheets/:id`        | Delete user profile     | Yes  |
| POST   | `/users_auth`              | Create user credentials | Yes  |
| GET    | `/users_auth/:id`          | Get user credentials    | Yes  |
| DELETE | `/users_auth/:id`          | Delete user credentials | Yes  |
| PATCH  | `/users_auth/:id/email`    | Update email            | Yes  |
| PATCH  | `/users_auth/:id/password` | Update password         | Yes  |

**User Story**: As an administrator, I need to manage user accounts to maintain the platform user base.

---

### Administration

| Method | Route                                 | Description                | Auth |
| ------ | ------------------------------------- | -------------------------- | ---- |
| PATCH  | `/admin/users/:id/deactivate`         | Deactivate user account    | Yes  |
| PATCH  | `/admin/users/:id/activate`           | Activate user account      | Yes  |
| POST   | `/admin/security/revoke-all-sessions` | Revoke all active sessions | Yes  |

**User Story**: As an administrator, I need security controls to manage user access and respond to security incidents.

---

### Classes

| Method | Route          | Description       | Auth |
| ------ | -------------- | ----------------- | ---- |
| GET    | `/classes`     | List all classes  | Yes  |
| POST   | `/classes`     | Create class      | Yes  |
| GET    | `/classes/:id` | Get class details | Yes  |
| PUT    | `/classes/:id` | Update class      | Yes  |
| DELETE | `/classes/:id` | Delete class      | Yes  |

**User Story**: As an administrator, I need to organize students into classes for course assignment and tracking.

---

### Skills

| Method | Route         | Description       | Auth |
| ------ | ------------- | ----------------- | ---- |
| GET    | `/skills`     | List all skills   | Yes  |
| POST   | `/skills`     | Create skill      | Yes  |
| GET    | `/skills/:id` | Get skill details | Yes  |
| PUT    | `/skills/:id` | Update skill      | Yes  |
| DELETE | `/skills/:id` | Delete skill      | Yes  |

**User Story**: As a teacher, I need to define skills that students will acquire through projects and coursework.

---

### Courses

| Method | Route          | Description        | Auth |
| ------ | -------------- | ------------------ | ---- |
| GET    | `/courses`     | List all courses   | Yes  |
| POST   | `/courses`     | Create course      | Yes  |
| GET    | `/courses/:id` | Get course details | Yes  |
| PUT    | `/courses/:id` | Update course      | Yes  |
| DELETE | `/courses/:id` | Delete course      | Yes  |

**User Story**: As an administrator, I need to create and manage courses that group related projects and skills.

---

### Projects

| Method | Route                   | Description             | Auth |
| ------ | ----------------------- | ----------------------- | ---- |
| GET    | `/projects`             | List all projects       | Yes  |
| POST   | `/projects`             | Create project          | Yes  |
| GET    | `/projects/:id`         | Get project details     | Yes  |
| PUT    | `/projects/:id`         | Update project          | Yes  |
| DELETE | `/projects/:id`         | Delete project          | Yes  |
| GET    | `/courses/:id/projects` | List projects by course | Yes  |

**User Story**: As a teacher, I need to create projects within courses that allow students to demonstrate skills.

---

### Steps

| Method | Route                 | Description           | Auth |
| ------ | --------------------- | --------------------- | ---- |
| GET    | `/steps`              | List all steps        | Yes  |
| POST   | `/steps`              | Create step           | Yes  |
| GET    | `/steps/:id`          | Get step details      | Yes  |
| PUT    | `/steps/:id`          | Update step           | Yes  |
| DELETE | `/steps/:id`          | Delete step           | Yes  |
| GET    | `/projects/:id/steps` | List steps by project | Yes  |

**User Story**: As a teacher, I need to break down projects into steps to guide students through skill acquisition.

---

### User-Class Assignments

| Method | Route                               | Description            | Auth |
| ------ | ----------------------------------- | ---------------------- | ---- |
| POST   | `/user-classes`                     | Assign user to class   | Yes  |
| GET    | `/users/:id/classes`                | Get user's classes     | Yes  |
| GET    | `/classes/:id/users`                | Get class members      | Yes  |
| DELETE | `/users/:user_id/classes/:class_id` | Remove user from class | Yes  |

**User Story**: As an administrator, I need to enroll students in classes to manage their course access.

---

### User-Course Enrollments

| Method | Route                                | Description             | Auth |
| ------ | ------------------------------------ | ----------------------- | ---- |
| POST   | `/user-courses`                      | Enroll user in course   | Yes  |
| GET    | `/users/:id/courses`                 | Get user's courses      | Yes  |
| GET    | `/courses/:id/users`                 | Get course participants | Yes  |
| DELETE | `/users/:user_id/courses/:course_id` | Remove user from course | Yes  |

**User Story**: As an administrator, I need to manage course enrollments to control student access to learning materials.

---

### Course-Class Associations

| Method | Route                                   | Description              | Auth |
| ------ | --------------------------------------- | ------------------------ | ---- |
| POST   | `/course-classes`                       | Link course to class     | Yes  |
| GET    | `/courses/:id/classes`                  | Get course's classes     | Yes  |
| GET    | `/classes/:id/courses`                  | Get class's courses      | Yes  |
| DELETE | `/courses/:course_id/classes/:class_id` | Unlink course from class | Yes  |

**User Story**: As an administrator, I need to assign courses to specific classes to manage curriculum delivery.

---

### Course-Skill Associations

| Method | Route                                  | Description              | Auth |
| ------ | -------------------------------------- | ------------------------ | ---- |
| POST   | `/course-skills`                       | Link skill to course     | Yes  |
| GET    | `/courses/:id/skills`                  | Get course's skills      | Yes  |
| GET    | `/skills/:id/courses`                  | Get skill's courses      | Yes  |
| DELETE | `/courses/:course_id/skills/:skill_id` | Unlink skill from course | Yes  |

**User Story**: As a teacher, I need to define which skills are covered by each course for curriculum planning.

---

### Step-Skill Associations

| Method | Route                              | Description            | Auth |
| ------ | ---------------------------------- | ---------------------- | ---- |
| POST   | `/step-skills`                     | Link skill to step     | Yes  |
| GET    | `/steps/:id/skills`                | Get step's skills      | Yes  |
| GET    | `/skills/:id/steps`                | Get skill's steps      | Yes  |
| DELETE | `/steps/:step_id/skills/:skill_id` | Unlink skill from step | Yes  |

**User Story**: As a teacher, I need to map skills to project steps to track which skills are demonstrated at each stage.

---

### Skill Validations

| Method | Route                                   | Description              | Auth |
| ------ | --------------------------------------- | ------------------------ | ---- |
| POST   | `/skill-validations`                    | Create validation record | Yes  |
| GET    | `/skill-validations/user/:user_id`      | Get user's validations   | Yes  |
| GET    | `/skill-validations/pending`            | List pending validations | Yes  |
| GET    | `/skill-validations/:user_id/:skill_id` | Get validation details   | Yes  |
| PATCH  | `/skill-validations/:user_id/:skill_id` | Update validation status | Yes  |
| DELETE | `/skill-validations/:user_id/:skill_id` | Delete validation        | Yes  |

**User Story**: As a teacher, I need to validate student skills and provide feedback to track their progression.

---

## Data Transfer Objects

### Authentication

```rust
LoginRequest { email: String, pwd: String }
AuthTokenResponse { token: String, token_type: String }
```

### Users

```rust
CreateUserSheet { nom: String, prenom: String, type_user: String, avatar: Option<String> }
UpdateUserSheet { nom: Option<String>, prenom: Option<String>, type_user: Option<String>, avatar: Option<String> }
CreateUserAuth { email: String, pwd: String, id_user_sheet: i32 }
```

### Classes

```rust
CreateClass { name: String, description: Option<String>, year: i32 }
UpdateClass { name: Option<String>, description: Option<String>, year: Option<i32> }
```

### Skills

```rust
CreateSkill { name: String, description: Option<String> }
UpdateSkill { name: Option<String>, description: Option<String> }
```

### Courses

```rust
CreateCourse { name: String, description: Option<String>, year: Option<i32> }
UpdateCourse { name: Option<String>, description: Option<String>, year: Option<i32> }
```

### Projects

```rust
CreateProject { name: String, description: Option<String>, course_id: i32 }
UpdateProject { name: Option<String>, description: Option<String>, course_id: Option<i32> }
```

### Steps

```rust
CreateStep { name: String, description: Option<String>, order_index: i32, project_id: i32 }
UpdateStep { name: Option<String>, description: Option<String>, order_index: Option<i32> }
```

### Skill Validations

```rust
CreateValidationRequest { user_id: i32, skill_id: i32, status: String, comment: Option<String> }
UpdateValidationStatus { status: String, comment: Option<String> }
```

---

## Role-Based Access Control

The API implements three user roles with distinct permissions:

| Role        | Permissions                                                            |
| ----------- | ---------------------------------------------------------------------- |
| **admin**   | Full access to all resources, user management, security controls       |
| **teacher** | Manage courses, projects, steps, skills; validate student competencies |
| **student** | View own profile, courses, and skill validations (read-only)           |

---

## Testing

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test skill_validations_test -- --test-threads=1

# Run with output
cargo test -- --nocapture
```

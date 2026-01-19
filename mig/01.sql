-- Migration 02: Complete Database Schema from MCD
-- This migration creates all tables for the Diplomind project

DROP TABLE IF EXISTS skill_validations;
DROP TABLE IF EXISTS step_skills;
DROP TABLE IF EXISTS user_classes;
DROP TABLE IF EXISTS user_courses;
DROP TABLE IF EXISTS course_classes;
DROP TABLE IF EXISTS course_skills;
DROP TABLE IF EXISTS steps;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS skills;
DROP TABLE IF EXISTS classes;
DROP TABLE IF EXISTS refresh_tokens;
DROP TABLE IF EXISTS users_auth;
DROP TABLE IF EXISTS users_sheets;

-- ============================================
-- 2. Core Entity Tables
-- ============================================

-- users sheets
CREATE TABLE IF NOT EXISTS users_sheets (
    id SERIAL PRIMARY KEY,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,
    type_user TEXT NOT NULL,
    profile_picture TEXT,
    active BOOLEAN DEFAULT TRUE
);

-- users auth
CREATE TABLE IF NOT EXISTS users_auth (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    pwd TEXT NOT NULL,
    id_user_sheet INTEGER REFERENCES users_sheets(id) ON DELETE CASCADE
);

-- refresh tokens
CREATE TABLE IF NOT EXISTS refresh_tokens (
    token TEXT NOT NULL PRIMARY KEY,
    id_user_auth INTEGER REFERENCES users_auth(id) ON DELETE CASCADE,
    expiration_date TIMESTAMPTZ NOT NULL 
);


-- Skills (Compétences)
CREATE TABLE IF NOT EXISTS skills (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Classes
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Courses (Cours)
CREATE TABLE IF NOT EXISTS courses (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Projects (Projets)
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Steps (Étapes de projet)
CREATE TABLE IF NOT EXISTS steps (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    step_order INTEGER DEFAULT 0, -- Pour ordonner les étapes
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 3. Association Tables (Many-to-Many)
-- ============================================

-- Skill Validations (VALIDATE in MCD)
-- Tracks which skills have been validated for each user
CREATE TABLE IF NOT EXISTS skill_validations (
    user_id INTEGER NOT NULL REFERENCES users_sheets(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'validated', 'rejected')),
    comment TEXT, -- Optional comment from teacher/admin
    validated_at TIMESTAMPTZ,
    validated_by INTEGER REFERENCES users_sheets(id), -- Teacher/Admin who validated
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, skill_id)
);

-- Step Skills (ETAPE_COMPETENCE in MCD)
-- Links skills to project steps
CREATE TABLE IF NOT EXISTS step_skills (
    step_id INTEGER NOT NULL REFERENCES steps(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    PRIMARY KEY (step_id, skill_id)
);

-- User Classes (CLASSE_UTILISATEUR in MCD)
-- Assigns users to classes
CREATE TABLE IF NOT EXISTS user_classes (
    user_id INTEGER NOT NULL REFERENCES users_sheets(id) ON DELETE CASCADE,
    class_id INTEGER NOT NULL REFERENCES classes(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, class_id)
);

-- User Courses (COURS_UTILISATEUR in MCD)
-- Assigns users to courses
CREATE TABLE IF NOT EXISTS user_courses (
    user_id INTEGER NOT NULL REFERENCES users_sheets(id) ON DELETE CASCADE,
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, course_id)
);

-- Course Classes (COURS_CLASSE in MCD)
-- Links courses to classes
CREATE TABLE IF NOT EXISTS course_classes (
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    class_id INTEGER NOT NULL REFERENCES classes(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, class_id)
);

-- Course Skills (COMPETENCE_COURS in MCD)
-- Links skills to courses
CREATE TABLE IF NOT EXISTS course_skills (
    course_id INTEGER NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, skill_id)
);

-- ============================================
-- 4. Indexes for Performance
-- ============================================

-- Users
CREATE INDEX IF NOT EXISTS idx_users_sheets_active ON users_sheets(active);
CREATE INDEX IF NOT EXISTS idx_users_auth_email ON users_auth(email);

-- Refresh Tokens
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expiration ON refresh_tokens(expiration_date);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_auth ON refresh_tokens(id_user_auth);

-- Skills
CREATE INDEX IF NOT EXISTS idx_skills_name ON skills(name);

-- Projects
CREATE INDEX IF NOT EXISTS idx_projects_course_id ON projects(course_id);

-- Steps
CREATE INDEX IF NOT EXISTS idx_steps_project_id ON steps(project_id);
CREATE INDEX IF NOT EXISTS idx_steps_order ON steps(project_id, step_order);

-- Skill Validations
CREATE INDEX IF NOT EXISTS idx_skill_validations_user ON skill_validations(user_id);
CREATE INDEX IF NOT EXISTS idx_skill_validations_skill ON skill_validations(skill_id);
CREATE INDEX IF NOT EXISTS idx_skill_validations_status ON skill_validations(status);

-- User Classes
CREATE INDEX IF NOT EXISTS idx_user_classes_user ON user_classes(user_id);
CREATE INDEX IF NOT EXISTS idx_user_classes_class ON user_classes(class_id);

-- User Courses
CREATE INDEX IF NOT EXISTS idx_user_courses_user ON user_courses(user_id);
CREATE INDEX IF NOT EXISTS idx_user_courses_course ON user_courses(course_id);

-- Course Classes
CREATE INDEX IF NOT EXISTS idx_course_classes_course ON course_classes(course_id);
CREATE INDEX IF NOT EXISTS idx_course_classes_class ON course_classes(class_id);

-- Course Skills
CREATE INDEX IF NOT EXISTS idx_course_skills_course ON course_skills(course_id);
CREATE INDEX IF NOT EXISTS idx_course_skills_skill ON course_skills(skill_id);

-- Step Skills
CREATE INDEX IF NOT EXISTS idx_step_skills_step ON step_skills(step_id);
CREATE INDEX IF NOT EXISTS idx_step_skills_skill ON step_skills(skill_id);

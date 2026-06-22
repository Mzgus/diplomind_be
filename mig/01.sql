-- Migration 01: Complete Database Schema from MCD (Updated for Multi-Account)
-- This migration creates all tables for the Diplomind project

-- Drop tables in correct order (dependents first)
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
DROP TABLE IF EXISTS accounts_users_sheets;
DROP TABLE IF EXISTS users_auth; -- Depends on accounts
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS users_sheets;

-- ============================================
-- 2. Core Entity Tables
-- ============================================

-- Accounts (New Root Identity)
CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL
);

-- Users Sheets (Profiles)
CREATE TABLE IF NOT EXISTS users_sheets (
    id SERIAL PRIMARY KEY,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,
    type_user TEXT NOT NULL CHECK (type_user IN ('admin', 'teacher', 'student')),
    profile_picture TEXT,
    active BOOLEAN DEFAULT TRUE
);

-- Association Accounts <-> Users Sheets
CREATE TABLE IF NOT EXISTS accounts_users_sheets (
    account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    user_sheet_id INTEGER NOT NULL REFERENCES users_sheets(id) ON DELETE CASCADE,
    PRIMARY KEY (account_id, user_sheet_id)
);

-- Users Auth (Credentials) - Linked to Account
CREATE TABLE IF NOT EXISTS users_auth (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    pwd TEXT NOT NULL,
    account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);

-- Refresh Tokens - Linked to Account (Session)
CREATE TABLE IF NOT EXISTS refresh_tokens (
    token TEXT NOT NULL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
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

-- Accounts
CREATE INDEX IF NOT EXISTS idx_accounts_email ON accounts(email);

-- Users
CREATE INDEX IF NOT EXISTS idx_users_sheets_active ON users_sheets(active);
CREATE INDEX IF NOT EXISTS idx_users_auth_email ON users_auth(email);

-- Accounts Users Sheets
CREATE INDEX IF NOT EXISTS idx_accounts_users_sheets_account ON accounts_users_sheets(account_id);
CREATE INDEX IF NOT EXISTS idx_accounts_users_sheets_sheet ON accounts_users_sheets(user_sheet_id);

-- Refresh Tokens
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expiration ON refresh_tokens(expiration_date);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_account ON refresh_tokens(account_id);

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

-- ============================================
-- 5. Data insertion basic admin
-- ============================================

INSERT INTO accounts (email) VALUES ('admin@diplomind.fr');
INSERT INTO users_sheets (last_name, first_name, type_user, profile_picture, active) VALUES ('Admin', 'Admin', 'admin', '', TRUE);
INSERT INTO users_auth (email, pwd, account_id) VALUES ('admin@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 1);
INSERT INTO accounts_users_sheets (account_id, user_sheet_id) VALUES (1, 1);

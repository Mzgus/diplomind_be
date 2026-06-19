-- Seed Data for Diplomind Database
-- This file populates the database with realistic test data
-- Run this AFTER running 01.sql and 02.sql migrations

-- Clear existing data (order matters for FKs)
TRUNCATE TABLE accounts_users_sheets CASCADE;
TRUNCATE TABLE accounts CASCADE;
-- Other tables are truncated by 01.sql DROP/CREATE usually, but if this is just seed:
-- TRUNCATE TABLE users_auth CASCADE; -- users_auth now depends on accounts
-- TRUNCATE TABLE users_sheets CASCADE;

-- Note: In a real seed overwrite specific to this migration structure, we assume tables are empty or we append.
-- Ensuring clean state for seed:
TRUNCATE skill_validations, step_skills, user_classes, user_courses, course_classes, course_skills, steps, projects, courses, skills, classes, refresh_tokens, accounts_users_sheets, users_auth, accounts, users_sheets RESTART IDENTITY CASCADE;


-- ============================================
-- 1. Users Sheets (Profiles)
-- ============================================

-- Admin users (id 1, 2)
INSERT INTO users_sheets (id, last_name, first_name, type_user, profile_picture, active) VALUES
(1, 'Martin', 'Sophie', 'admin', 'https://i.pravatar.cc/150?img=1', TRUE),
(2, 'Bernard', 'Lucas', 'admin', 'https://i.pravatar.cc/150?img=2', TRUE);

-- Teachers (id 3, 4, 5)
INSERT INTO users_sheets (id, last_name, first_name, type_user, profile_picture, active) VALUES
(3, 'Dubois', 'Marie', 'teacher', 'https://i.pravatar.cc/150?img=5', TRUE),
(4, 'Petit', 'Jean', 'teacher', 'https://i.pravatar.cc/150?img=6', TRUE),
(5, 'Robert', 'Claire', 'teacher', 'https://i.pravatar.cc/150?img=7', TRUE);

-- Students (id 6-15)
INSERT INTO users_sheets (id, last_name, first_name, type_user, profile_picture, active) VALUES
(6, 'Moreau', 'Emma', 'student', 'https://i.pravatar.cc/150?img=10', TRUE),
(7, 'Simon', 'Louis', 'student', 'https://i.pravatar.cc/150?img=11', TRUE),
(8, 'Laurent', 'Chloé', 'student', 'https://i.pravatar.cc/150?img=12', TRUE),
(9, 'Lefebvre', 'Hugo', 'student', 'https://i.pravatar.cc/150?img=13', TRUE),
(10, 'Michel', 'Léa', 'student', 'https://i.pravatar.cc/150?img=14', TRUE),
(11, 'Garcia', 'Nathan', 'student', 'https://i.pravatar.cc/150?img=15', TRUE),
(12, 'David', 'Manon', 'student', 'https://i.pravatar.cc/150?img=16', TRUE),
(13, 'Bertrand', 'Tom', 'student', 'https://i.pravatar.cc/150?img=17', TRUE),
(14, 'Roux', 'Sarah', 'student', 'https://i.pravatar.cc/150?img=18', TRUE),
(15, 'Vincent', 'Arthur', 'student', 'https://i.pravatar.cc/150?img=19', TRUE),

-- Double Identity User (id 16, 17)
(16, 'Polyvalent', 'Alex', 'teacher', 'https://i.pravatar.cc/150?img=20', TRUE),
(17, 'Polyvalent', 'Alex', 'student', 'https://i.pravatar.cc/150?img=21', TRUE);


-- ============================================
-- 2. Accounts & Auth
-- ============================================

-- Accounts (Identities)
INSERT INTO accounts (id, email) VALUES
(1, 'sophie.martin@diplomind.fr'),
(2, 'lucas.bernard@diplomind.fr'),
(3, 'marie.dubois@diplomind.fr'),
(4, 'jean.petit@diplomind.fr'),
(5, 'claire.robert@diplomind.fr'),
(6, 'emma.moreau@student.diplomind.fr'),
(7, 'louis.simon@student.diplomind.fr'),
(8, 'chloe.laurent@student.diplomind.fr'),
(9, 'hugo.lefebvre@student.diplomind.fr'),
(10, 'lea.michel@student.diplomind.fr'),
(11, 'nathan.garcia@student.diplomind.fr'),
(12, 'manon.david@student.diplomind.fr'),
(13, 'tom.bertrand@student.diplomind.fr'),
(14, 'sarah.roux@student.diplomind.fr'),
(15, 'arthur.vincent@student.diplomind.fr'),
(16, 'alex.poly@diplomind.fr');

-- Users Auth (Credentials) - Linked to Accounts
-- Password: "Password123" hashed with Argon2
INSERT INTO users_auth (email, pwd, account_id) VALUES
('sophie.martin@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 1),
('lucas.bernard@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 2),
('marie.dubois@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 3),
('jean.petit@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 4),
('claire.robert@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 5),
('emma.moreau@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 6),
('louis.simon@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 7),
('chloe.laurent@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 8),
('hugo.lefebvre@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 9),
('lea.michel@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 10),
('nathan.garcia@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 11),
('manon.david@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 12),
('tom.bertrand@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 13),
('sarah.roux@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 14),
('arthur.vincent@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 15),
('alex.poly@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 16);

-- Link Accounts to User Sheets (1-to-1 default for now)
INSERT INTO accounts_users_sheets (account_id, user_sheet_id) VALUES
(1, 1), (2, 2), (3, 3), (4, 4), (5, 5),
(6, 6), (7, 7), (8, 8), (9, 9), (10, 10),
(11, 11), (12, 12), (13, 13), (14, 14), (15, 15),
-- Multi-Account Link: Alex Poly has 2 profiles (16: Teacher, 17: Student)
(16, 16), (16, 17);


-- ============================================
-- 3. Classes
-- ============================================

INSERT INTO classes (name) VALUES
('CDA 2024-2025'),
('Développement Web Full Stack'),
('DevOps & Cloud'),
('Data Science & IA');

-- ============================================
-- 4. Courses
-- ============================================

INSERT INTO courses (name, description) VALUES
('Backend Development avec Rust', 'Apprendre à construire des APIs robustes et performantes avec Rust et Poem'),
('Frontend React Avancé', 'Maîtriser React, TypeScript et les patterns modernes'),
('Base de Données PostgreSQL', 'Conception, optimisation et administration de bases de données'),
('Architecture Microservices', 'Concevoir et déployer des architectures distribuées'),
('DevOps & CI/CD', 'Automatisation, conteneurisation et déploiement continu'),
('Sécurité des Applications Web', 'OWASP, authentification, autorisation et bonnes pratiques');

-- ============================================
-- 5. Skills (Compétences)
-- ============================================

INSERT INTO skills (name, description) VALUES
-- Backend
('Rust Programming', 'Maîtrise du langage Rust et de son écosystème'),
('API RESTful Design', 'Conception d''APIs REST suivant les bonnes pratiques'),
('PostgreSQL', 'Conception et optimisation de bases de données relationnelles'),
('Authentication & Authorization', 'Implémentation de systèmes d''authentification sécurisés (JWT, OAuth)'),
('Error Handling', 'Gestion robuste des erreurs et logging'),

-- Frontend
('React & TypeScript', 'Développement d''interfaces modernes avec React et TypeScript'),
('State Management', 'Gestion d''état avec Redux, Zustand ou Context API'),
('Responsive Design', 'Création d''interfaces adaptatives'),

-- DevOps
('Docker & Containerization', 'Conteneurisation d''applications'),
('CI/CD Pipelines', 'Mise en place de pipelines d''intégration continue'),
('Cloud Deployment', 'Déploiement sur AWS, GCP ou Azure'),

-- Soft Skills
('Code Review', 'Revue de code et collaboration en équipe'),
('Git & Version Control', 'Maîtrise de Git et des workflows collaboratifs'),
('Agile Methodology', 'Travail en méthodologie Agile/Scrum'),
('Technical Documentation', 'Rédaction de documentation technique claire');

-- ============================================
-- 6. Projects
-- ============================================

INSERT INTO projects (name, description, course_id) VALUES
('API Diplomind', 'Développement de l''API backend pour la plateforme Diplomind', 1),
('Dashboard Étudiant', 'Interface de suivi des compétences pour les étudiants', 2),
('Système de Validation', 'Module de validation des compétences par les enseignants', 1),
('Migration PostgreSQL', 'Migration et optimisation du schéma de base de données', 3),
('Pipeline CI/CD', 'Mise en place de l''automatisation des tests et déploiements', 5);

-- ============================================
-- 7. Steps (Étapes de projet)
-- ============================================

-- Projet 1: API Diplomind
INSERT INTO steps (name, description, project_id, step_order) VALUES
('Setup du projet Rust', 'Initialisation du projet avec Cargo et dépendances', 1, 1),
('Modèles de données', 'Création des structs et modèles Rust', 1, 2),
('Couche base de données', 'Implémentation des requêtes SQL avec sqlx', 1, 3),
('Handlers HTTP', 'Création des endpoints REST', 1, 4),
('Authentification JWT', 'Mise en place du système d''authentification', 1, 5),
('Tests unitaires', 'Écriture des tests pour chaque module', 1, 6);

-- Projet 2: Dashboard Étudiant
INSERT INTO steps (name, description, project_id, step_order) VALUES
('Setup React + TypeScript', 'Initialisation du projet frontend', 2, 1),
('Composants UI', 'Création des composants réutilisables', 2, 2),
('Intégration API', 'Connexion avec l''API backend', 2, 3),
('Gestion d''état', 'Mise en place du state management', 2, 4),
('Responsive Design', 'Adaptation mobile et tablette', 2, 5);

-- Projet 3: Système de Validation
INSERT INTO steps (name, description, project_id, step_order) VALUES
('Modèle de validation', 'Conception du système de statuts', 3, 1),
('Interface enseignant', 'Création de l''interface de validation', 3, 2),
('Notifications', 'Système de notifications pour les étudiants', 3, 3);

-- ============================================
-- 8. Link Skills to Steps
-- ============================================

-- Projet 1: API Diplomind
INSERT INTO step_skills (step_id, skill_id) VALUES
(1, 1), (1, 13), -- Setup: Rust + Git
(2, 1), (2, 3),  -- Modèles: Rust + PostgreSQL
(3, 1), (3, 3), (3, 5), -- DB: Rust + PostgreSQL + Error Handling
(4, 1), (4, 2), -- Handlers: Rust + API Design
(5, 1), (5, 4), -- Auth: Rust + Auth
(6, 1), (6, 12); -- Tests: Rust + Code Review

-- Projet 2: Dashboard
INSERT INTO step_skills (step_id, skill_id) VALUES
(7, 6), (7, 13),  -- Setup: React + Git
(8, 6), (8, 8),   -- Composants: React + Responsive
(9, 6), (9, 2),   -- API: React + API Design
(10, 6), (10, 7), -- State: React + State Management
(11, 6), (11, 8); -- Responsive: React + Responsive Design

-- ============================================
-- 9. Link Courses to Classes
-- ============================================

INSERT INTO course_classes (course_id, class_id) VALUES
(1, 1), -- Backend Rust → CDA
(2, 1), -- Frontend React → CDA
(2, 2), -- Frontend React → Web Full Stack
(3, 1), -- PostgreSQL → CDA
(4, 1), -- Microservices → CDA
(5, 3), -- DevOps → DevOps & Cloud
(6, 1), (6, 2); -- Sécurité → CDA + Web Full Stack

-- ============================================
-- 10. Link Skills to Courses
-- ============================================

INSERT INTO course_skills (course_id, skill_id) VALUES
-- Backend Development
(1, 1), (1, 2), (1, 3), (1, 4), (1, 5),
-- Frontend React
(2, 6), (2, 7), (2, 8),
-- PostgreSQL
(3, 3), (3, 15),
-- Microservices
(4, 1), (4, 2), (4, 9),
-- DevOps
(5, 9), (5, 10), (5, 11),
-- Sécurité
(6, 4), (6, 5);

-- ============================================
-- 11. Assign Students to Classes
-- ============================================

-- CDA 2024-2025 (tous les étudiants)
INSERT INTO user_classes (user_id, class_id) VALUES
(6, 1), (7, 1), (8, 1), (9, 1), (10, 1),
(11, 1), (12, 1), (13, 1), (14, 1), (15, 1);

-- Web Full Stack (quelques étudiants)
INSERT INTO user_classes (user_id, class_id) VALUES
(6, 2), (7, 2), (9, 2), (11, 2), (13, 2);

-- ============================================
-- 12. Assign Teachers to Courses
-- ============================================
-- Note: users_sheets IDs for teachers: 3=Marie, 4=Jean, 5=Claire, 16=Alex

-- Backend Development (course 1)
INSERT INTO user_courses (user_id, course_id) VALUES
(3, 1), (4, 1);

-- Frontend React (course 2)
INSERT INTO user_courses (user_id, course_id) VALUES
(4, 2), (5, 2);

-- PostgreSQL (course 3)
INSERT INTO user_courses (user_id, course_id) VALUES
(3, 3), (5, 3);

-- Architecture Microservices (course 4)
INSERT INTO user_courses (user_id, course_id) VALUES
(4, 4), (16, 4);

-- DevOps & CI/CD (course 5)
INSERT INTO user_courses (user_id, course_id) VALUES
(5, 5), (16, 5);

-- Sécurité des Applications Web (course 6)
INSERT INTO user_courses (user_id, course_id) VALUES
(3, 6), (16, 6);

-- ============================================
-- 13. Skill Validations
-- ============================================

-- Emma (user sheet_id=6) - Étudiante avancée
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(6, 1, 'validated', NOW() - INTERVAL '10 days', 4), -- Rust validé par Marie
(6, 2, 'validated', NOW() - INTERVAL '8 days', 4),  -- API Design validé
(6, 3, 'validated', NOW() - INTERVAL '5 days', 5),  -- PostgreSQL validé par Jean
(6, 6, 'pending', NULL, NULL),                       -- React en attente
(6, 13, 'validated', NOW() - INTERVAL '15 days', 4); -- Git validé

-- Louis (user sheet_id=7) - Étudiant avancé
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(7, 1, 'validated', NOW() - INTERVAL '10 days', 4),
(7, 2, 'validated', NOW() - INTERVAL '8 days', 4),
(7, 3, 'validated', NOW() - INTERVAL '5 days', 5),
(7, 6, 'pending', NULL, NULL),
(7, 13, 'validated', NOW() - INTERVAL '15 days', 4);

-- Louis (user 8) - Étudiant moyen
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(8, 1, 'validated', NOW() - INTERVAL '7 days', 4),
(8, 3, 'pending', NULL, NULL),
(8, 13, 'validated', NOW() - INTERVAL '12 days', 5);

-- Chloé (user 9) - Étudiante débutante
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(9, 1, 'pending', NULL, NULL),
(9, 13, 'validated', NOW() - INTERVAL '5 days', 4);

-- Hugo (user 10) - Quelques validations
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(10, 1, 'validated', NOW() - INTERVAL '3 days', 5),
(10, 3, 'validated', NOW() - INTERVAL '2 days', 5),
(10, 5, 'rejected', NOW() - INTERVAL '1 day', 4); -- Error Handling rejeté

-- ============================================
-- 14. Reset Sequences
-- ============================================

SELECT setval('users_sheets_id_seq', COALESCE((SELECT MAX(id) FROM users_sheets), 1), true);
SELECT setval('accounts_id_seq', COALESCE((SELECT MAX(id) FROM accounts), 1), true);
SELECT setval('users_auth_id_seq', COALESCE((SELECT MAX(id) FROM users_auth), 1), true);
SELECT setval('classes_id_seq', COALESCE((SELECT MAX(id) FROM classes), 1), true);
SELECT setval('courses_id_seq', COALESCE((SELECT MAX(id) FROM courses), 1), true);
SELECT setval('skills_id_seq', COALESCE((SELECT MAX(id) FROM skills), 1), true);
SELECT setval('projects_id_seq', COALESCE((SELECT MAX(id) FROM projects), 1), true);
SELECT setval('steps_id_seq', COALESCE((SELECT MAX(id) FROM steps), 1), true);

-- Seed completed!
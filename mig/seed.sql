-- Seed Data for Diplomind Database
-- This file populates the database with realistic test data
-- Run this AFTER running 01.sql migration

-- Reset sequences to start from 1
SELECT setval('users_sheets_id_seq', 1, false);
SELECT setval('users_auth_id_seq', 1, false);

-- ============================================
-- 1. Users (Students, Teachers, Admins)
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
(15, 'Vincent', 'Arthur', 'student', 'https://i.pravatar.cc/150?img=19', TRUE);

-- Authentication for all users (password: "Password123" hashed with Argon2)
-- Admin
INSERT INTO users_auth (email, pwd, id_user_sheet) VALUES
('sophie.martin@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 2),
('lucas.bernard@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 3);

-- Teachers (id 3, 4, 5)
INSERT INTO users_auth (email, pwd, id_user_sheet) VALUES
('marie.dubois@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 3),
('jean.petit@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 4),
('claire.robert@diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 5);

-- Students (id 6-15)
INSERT INTO users_auth (email, pwd, id_user_sheet) VALUES
('emma.moreau@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 6),
('louis.simon@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 7),
('chloe.laurent@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 8),
('hugo.lefebvre@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 9),
('lea.michel@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 10),
('nathan.garcia@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 11),
('manon.david@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 12),
('tom.bertrand@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 13),
('sarah.roux@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 14),
('arthur.vincent@student.diplomind.fr', '$argon2id$v=19$m=19456,t=2,p=1$GhcmbW6yjuETRE7GbhZz6A$HmwWF+GRl3vD0A2+7RuDkCcCGqUwblOtKJoEJI7/sSI', 15);

-- ============================================
-- 2. Classes
-- ============================================

INSERT INTO classes (name) VALUES
('CDA 2024-2025'),
('Développement Web Full Stack'),
('DevOps & Cloud'),
('Data Science & IA');

-- ============================================
-- 3. Courses
-- ============================================

INSERT INTO courses (name, description) VALUES
('Backend Development avec Rust', 'Apprendre à construire des APIs robustes et performantes avec Rust et Poem'),
('Frontend React Avancé', 'Maîtriser React, TypeScript et les patterns modernes'),
('Base de Données PostgreSQL', 'Conception, optimisation et administration de bases de données'),
('Architecture Microservices', 'Concevoir et déployer des architectures distribuées'),
('DevOps & CI/CD', 'Automatisation, conteneurisation et déploiement continu'),
('Sécurité des Applications Web', 'OWASP, authentification, autorisation et bonnes pratiques');

-- ============================================
-- 4. Skills (Compétences)
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
-- 5. Projects
-- ============================================

INSERT INTO projects (name, description, course_id) VALUES
('API Diplomind', 'Développement de l''API backend pour la plateforme Diplomind', 1),
('Dashboard Étudiant', 'Interface de suivi des compétences pour les étudiants', 2),
('Système de Validation', 'Module de validation des compétences par les enseignants', 1),
('Migration PostgreSQL', 'Migration et optimisation du schéma de base de données', 3),
('Pipeline CI/CD', 'Mise en place de l''automatisation des tests et déploiements', 5);

-- ============================================
-- 6. Steps (Étapes de projet)
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
-- 7. Link Skills to Steps
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
-- 8. Link Courses to Classes
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
-- 9. Link Skills to Courses
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
-- 10. Assign Students to Classes
-- ============================================

-- CDA 2024-2025 (tous les étudiants)
INSERT INTO user_classes (user_id, class_id) VALUES
(7, 1), (8, 1), (9, 1), (10, 1), (11, 1),
(12, 1), (13, 1), (14, 1), (15, 1), (16, 1);

-- Web Full Stack (quelques étudiants)
INSERT INTO user_classes (user_id, class_id) VALUES
(7, 2), (9, 2), (11, 2), (13, 2);

-- ============================================
-- 11. Assign Students to Courses
-- ============================================

-- Backend Development
INSERT INTO user_courses (user_id, course_id) VALUES
(7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1);

-- Frontend React
INSERT INTO user_courses (user_id, course_id) VALUES
(7, 2), (9, 2), (11, 2), (13, 2), (15, 2);

-- PostgreSQL
INSERT INTO user_courses (user_id, course_id) VALUES
(7, 3), (8, 3), (10, 3), (12, 3), (14, 3);

-- ============================================
-- 12. Skill Validations (quelques exemples)
-- ============================================

-- Emma (user 7) - Étudiante avancée
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(7, 1, 'validated', NOW() - INTERVAL '10 days', 4), -- Rust validé par Marie
(7, 2, 'validated', NOW() - INTERVAL '8 days', 4),  -- API Design validé
(7, 3, 'validated', NOW() - INTERVAL '5 days', 5),  -- PostgreSQL validé par Jean
(7, 6, 'pending', NULL, NULL),                       -- React en attente
(7, 13, 'validated', NOW() - INTERVAL '15 days', 4); -- Git validé

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
-- 13. Reset Sequences (important!)
-- ============================================

-- Ajuster les séquences pour éviter les conflits d'ID
SELECT setval('users_sheets_id_seq', (SELECT MAX(id) FROM users_sheets));
SELECT setval('users_auth_id_seq', (SELECT MAX(id) FROM users_auth));
SELECT setval('classes_id_seq', (SELECT MAX(id) FROM classes));
SELECT setval('courses_id_seq', (SELECT MAX(id) FROM courses));
SELECT setval('skills_id_seq', (SELECT MAX(id) FROM skills));
SELECT setval('projects_id_seq', (SELECT MAX(id) FROM projects));
SELECT setval('steps_id_seq', (SELECT MAX(id) FROM steps));

-- ============================================
-- Seed completed!
-- ============================================
-- Users: 16 (1 admin initial + 2 admins + 3 teachers + 10 students)
-- Classes: 4
-- Courses: 6
-- Skills: 15
-- Projects: 5
-- Steps: 14
-- Validations: 12 (various statuses)
-- passwords: Password123
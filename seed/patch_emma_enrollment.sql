-- Patch: Enroll Emma Moreau (users_sheets.id = 6) in courses, classes, and skill validations
-- Run this directly against the DB to fix missing data without a full reseed
-- Context: seed.sql had a comment error (called sheet_id=7 "Emma" — she is actually sheet_id=6)

-- Courses (Emma gets Backend, Frontend React, PostgreSQL)
INSERT INTO user_courses (user_id, course_id) VALUES
(6, 1), -- Backend Development avec Rust
(6, 2), -- Frontend React Avancé
(6, 3)  -- Base de Données PostgreSQL
ON CONFLICT DO NOTHING;

-- Classes (CDA 2024-2025 + Web Full Stack)
INSERT INTO user_classes (user_id, class_id) VALUES
(6, 1), -- CDA 2024-2025
(6, 2)  -- Développement Web Full Stack
ON CONFLICT DO NOTHING;

-- Skill Validations
INSERT INTO skill_validations (user_id, skill_id, status, validated_at, validated_by) VALUES
(6, 1, 'validated', NOW() - INTERVAL '10 days', 4), -- Rust
(6, 2, 'validated', NOW() - INTERVAL '8 days', 4),  -- API Design
(6, 3, 'validated', NOW() - INTERVAL '5 days', 5),  -- PostgreSQL
(6, 6, 'pending', NULL, NULL),                       -- React en attente
(6, 13, 'validated', NOW() - INTERVAL '15 days', 4) -- Git
ON CONFLICT DO NOTHING;

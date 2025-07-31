DELETE FROM project_repositories
WHERE project_id NOT IN (SELECT id FROM projects);

ALTER TABLE project_repositories
    ADD CONSTRAINT fk_project_repositories_project_id
        FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE project_repositories
    VALIDATE CONSTRAINT fk_project_repositories_project_id;

DELETE FROM project_repository_statuses
WHERE project_id NOT IN (SELECT id FROM projects);

ALTER TABLE project_repository_statuses
    ADD CONSTRAINT fk_project_repository_statuses_project_id
        FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE project_repository_statuses
    VALIDATE CONSTRAINT fk_project_repository_statuses_project_id;

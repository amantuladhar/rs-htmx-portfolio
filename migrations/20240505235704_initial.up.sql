CREATE TABLE IF NOT EXISTS rs_portfolio_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT rs_portfolio_user_uk_username UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS rs_portfolio_experience(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE null,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_rs_portfolio_experience_user_id FOREIGN KEY (user_id) REFERENCES rs_portfolio_user(id)
);

CREATE TABLE IF NOT EXISTS rs_portfolio_education(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    school_name VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE null,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_rs_portfolio_education_user_id FOREIGN KEY (user_id) REFERENCES rs_portfolio_user(id)
);

DO $$ BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            information_schema.tables
        WHERE
            table_schema = 'public'
            AND table_name = 'rs_portfolio_user'
    ) THEN
    INSERT INTO
        rs_portfolio_user (username, password)
    VALUES
        (
            'user1',
            '$argon2id$v=19$m=19456,t=2,p=1$Mprq5khdSq5rhCCPe3RcjA$dcFx+GAM7wpjXordwAvJGdxHbn8Gv7lRSrRbxZemimk'
        );

END IF;

END $$;

DO $$ BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            information_schema.tables
        WHERE
            table_schema = 'public'
            AND table_name = 'rs_portfolio_experience'
    ) THEN
    INSERT INTO
        rs_portfolio_experience (
            user_id,
            title,
            company,
            location,
            start_date,
            end_date,
            description
        )
    VALUES
        (
            1,
            'Senio Software Engineer',
            'Google',
            'Mountain View, CA',
            '2019-01-01',
            null,
            'Worked on the search team.'
        ),
        (
            1,
            'Intermediate Software Engineer',
            'Facebook',
            'Menlo Park, CA',
            '2018-01-01',
            '2019-01-01',
            'Worked on the news feed team.'
        ),
        (
            1,
            'Junior Software Engineer',
            'Microsoft',
            'Redmond, WA',
            '2017-01-01',
            '2018-01-01',
            'Worked on the Windows team.'
        );

END IF;

END $$;

DO $$ BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            information_schema.tables
        WHERE
            table_schema = 'public'
            AND table_name = 'rs_portfolio_education'
    ) THEN
    INSERT INTO
        rs_portfolio_education (
            user_id,
            title,
            school_name,
            location,
            start_date,
            end_date,
            description
        )
    VALUES
        (
            1,
            'Bachelor of Science in Computer Science',
            'MIT',
            'US',
            '2019-01-01',
            null,
            'blah blah blah'
        );

END IF;

END $$;
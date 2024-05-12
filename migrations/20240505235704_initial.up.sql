CREATE TABLE IF NOT EXISTS rs_portfolio_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT rs_portfolio_user_uk_username UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS rs_portfolio_experience(
    id serial primary key,
    user_id integer not null,
    title varchar(255) not null,
    company varchar(255) not null,
    location varchar(255) not null,
    start_date date not null,
    end_date date null,
    description text not null,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    constraint fk_rs_portfolio_experience_user_id foreign key (user_id) references rs_portfolio_user(id)
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
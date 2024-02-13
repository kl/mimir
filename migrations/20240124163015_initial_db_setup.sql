-- Create blog post table
create table if not exists posts(
    id integer primary key autoincrement not null,
    url_id text not null,
    title text not null,
    markdown text not null,
    html text not null,
    is_published integer not null, -- bool
    published_at integer, -- unix ts
    updated_at integer    -- unix ts
) strict;

create unique index index_posts_url_id
    on posts (url_id);

-- Create admin auth table
create table if not exists admin_auth(
    hashed_password text not null primary key
) strict;

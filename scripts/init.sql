CREATE TABLE WeatherStats (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                zip VARCHAR(255) NOT NULL,
                city VARCHAR(255) NOT NULL,
                temperature DOUBLE NOT NULL,
                weather VARCHAR(255) NOT NULL,
                humidity VARCHAR(255) NOT NULL,
                wind_speed DOUBLE NOT NULL,
                measurement_time DATETIME DEFAULT CURRENT_TIMESTAMP
            );



create table Users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);




create table Areas (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);



CREATE TABLE TargetZips (
    zip_code VARCHAR(10) PRIMARY KEY,
    area_id INT,
    FOREIGN KEY (area_id) REFERENCES Areas(id)
);


create table UserZips ( 
    zip_code VARCHAR(10),     
    user_id INT,     
    PRIMARY KEY (zip_code, user_id),     
    FOREIGN KEY (zip_code) REFERENCES TargetZips(zip_code),     
    FOREIGN KEY (user_id) REFERENCES Users(id) 
);


insert into Areas (name) VALUES ('San Jose'), ('Los Angeles');
insert into TargetZips (zip_code, area_id) VALUES ('95124', 1), ('95014', 1), ('95123', 1), ('90046', 2), ('90291',2);
insert into Users (id, name) VALUES (1, 'Bob'), (2, 'Joe'), (3, 'Dan');
insert into UserZips (zip_code, user_id) VALUES ('95124', 1), ('95014', 2), ('90046', 3), ('95124', 2);
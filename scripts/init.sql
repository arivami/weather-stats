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
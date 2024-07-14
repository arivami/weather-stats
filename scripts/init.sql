CREATE TABLE weather_data (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                zip VARCHAR(255) NOT NULL,
                city VARCHAR(255) NOT NULL,
                temperature DOUBLE NOT NULL,
                weather VARCHAR(255) NOT NULL,
                humidity VARCHAR(255) NOT NULL,
                wind_speed DOUBLE NOT NULL,
                measurement_time DATETIME DEFAULT CURRENT_TIMESTAMP
            );
# Weather Stats Service

Welcome to the Weather Stats Service! This project provides a platform for users to collect, view, and analyze weather data based on locations of interest. Users can specify zip codes they want to track, and the service will handle regular data collection, storage, and retrieval.

## Project Overview

### Architecture
1. **Back End**: Powered by **Amazon RDS MySQL**, the back end securely stores collected weather data as well as targets for future data collection.
2. **Middle Layer**: This layer contains the core business logic. It manages user inputs, handles scheduled weather data collection from third-party APIs, processes the data, and stores it in the database.
3. **Front End**: An Android application written in **Kotlin** *(currently under development)*. The app will feature:
   - A user interface to view weather data.
   - Functionality to add or modify target zip codes for data collection.
   - Support for multiple users, ensuring that each user only accesses the weather data they have targeted for collection.


### Features
- Scheduled data collection based on user-specified zip codes.
- User-friendly interface for managing weather data of tracked locations.

### Tech Stack
- **Rust**: For back-end logic and data processing.
- **Amazon RDS MySQL**: Database for storing weather data.
- **Docker**: Containerization for deployment.
- **Amazon ECS (Fargate)**: Running the service as a containerized task.
- **Amazon EventBridge**: Scheduling the ECS task.
- **Kotlin**: For Android app development.

### How to Run
You can run the service either locally for testing or in the cloud using AWS. Below are the steps for both environments. Implementing these steps requires some knowledge of Rust, Docker, AWS, and MySQL.

#### Local (for Testing)
1. Clone the repository.
2. Ensure Docker is up and running on your machine.
3. Set up environment variables:
    - Use the provided [.env.example](./.env.example) file as a template.
    - Configure any required variables in the [build.sh](./scripts/build.sh) script according to your local setup.
4. Run the [build.sh](./scripts/build.sh) script to build and start the containers:
    - Once the script completes, you will have two running containers: one for MySQL and one for the application.
    - *Note: Neither the service nor the DB run natively on your machine, local MySQL or Rust is not required.*
5. Use the service:
    - Attach a shell to the application container to run the service manually using `cargo run`.
6. View results:
    - Attach a shell to the MySQL container to interact with the database and view the tables.
    - The `TargetZips` table will contain some sample targets. After one run, the `WeatherStats` table will have entries.


#### Cloud Based ####
1. Sign in to **AWS** or create an AWS account and complete the setup.
2. Navigate to **ECS**:
    - Create a cluster with all standard settings.
    - Create a task definition with all standard settings, using `docker.io/azeltser/weather-stats:latest` as the Docker image.
3. Navigate to **RDS**:
    - Create a MySQL database with all standard settings.
5. Navigate to **EC2**: 
    - Create a standard instance. 
    - Navigate to networking options and connect the instance to the RDS MySQL DB.
    - Connect to the instance.
    - From the instance terminal, connect to your MySQL DB.
6. Clone the GitHub repository (keep the instance terminal open).
7. Copy and paste all the SQL statements from [init.sql](./scripts/init.sql) into the instance terminal to initialize the database.
8. Navigate back to **ECS**: 
    - Run the task
    - Check the results in the instance terminal by querying the DB.



### Future Plans
- Complete and integrate the Android front end.
- Enhance the service with additional data analytics features.
- Support more data sources for improved coverage.
- Potentially replace the scheduled ECS task with a scheduled AWS Lambda for greater efficiency and scalability.

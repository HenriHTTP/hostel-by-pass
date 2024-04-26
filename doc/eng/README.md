# About project
The  Hostel By Pass project is a microservice written in Rust and Axum framework, designed to provide various useful features for managing room reservations. The microservice uses MongoDB as its database.
this was created for stay easy the reservation management 

## Architecture
"The architecture of this project was designed with the aim of facilitating component interconnection and simplifying the addition of new features. This makes it easier and safer for developers and the business to perform maintenance and implement new ideas quickly. The project's structure is organized in layers, meaning that the different elements communicate with each other efficiently. Each component was created with a specific purpose, ensuring that each part of the project fulfills its function.
### Controller
The controllers are components created to handle validations and manage the flow of service requests directly. They can utilize important methods like those from the repository. In this project, each controller deals with only one use case, and its validations are kept private."
### Entity
An Entity is like a component created to represent an object or resource within a system. Each entity has its specific attributes and is responsible for encapsulating data and operations related to these objects, providing an organized and efficient way to interact with the system
### Repository
A "repository" is like a database abstraction where we store and manage essential operations. Each "repository" has a specific purpose and provides methods to add, update, and retrieve operations. They help keep the code organized and simplify operation manipulation, keeping them secure and accessible.
### Services
Services are abstractions of libraries or interfaces with easy-to-use methods, designed to avoid directly coupling third-party packages within the application. They act as an intermediary layer, making it easier for developers to access external resources without dealing directly with the complexity of those packages.
### Route
Routes are like pathways in a system, guiding requests to the right destination. They play a crucial role in handling incoming requests and directing them to the appropriate controllers, which are responsible for executing specific actions or operations. Essentially, routes act as the traffic controllers of a web application, ensuring that requests are properly processed and responded to according to the defined logic and functionality
### Helper
Helpers serve as reliable assistants within a system, offering practical solutions to common tasks across different areas. These methods are typically static, meaning they don't require an instance of an object to be used, making them easily accessible from anywhere within the application. Their versatility lies in their ability to perform diverse functions, such as formatting data, handling calculations, or validating inputs. By encapsulating these functionalities into reusable modules, helpers promote code reusability, maintainability, and overall efficiency in software development.
# Environment Requirements
To use the application that utilizes `Docker` and `Rust`, it is necessary to have a compatible operating system, such as Linux, and have `Docker` installed and configured. Users must have a basic understanding of Docker and its concepts, as well as a `Rust` development environment configured with the `Rustc` compiler and the `Cargo` package manager. After cloning the application repository, the Rust code must be compiled using `Cargo`. Then, users should build a Docker image from the Dockerfile provided in the repository and run a Docker container using the `docker run` command. After executing the container, users can test and validate the application functionality. It is important to monitor and maintain the application, performing regular maintenance of the operating system, Docker image, and the application itself to ensure continuous and reliable operation.

## why MongoDB?
Using MongoDB alongside Rust brings several benefits, especially in a microservices context. MongoDB is a highly scalable and flexible NoSQL database, while Rust is a programming language known for its safety and efficiency. The combination of these technologies enables seamless integration between Rust code and the database, making data manipulation and storage easier. Additionally, MongoDB's BSON format, which is similar to JSON, offers greater compatibility with JSON-formatted structured data, common in microservices architectures. This makes data exchange between services easier and more efficient. Furthermore, MongoDB's flexibility regarding changes in business rules is another significant advantage, allowing for quick adjustments and adaptations as needed without compromising data integrity or system stability. In summary, the combination of MongoDB and Rust provides a solid and flexible foundation for developing robust and scalable microservices.
# Json
Using JSON offers several benefits, primarily because it's the most commonly used format for sending requests. Its simplicity and readability make it ideal for transmitting data between systems, whether it's between a client and a server or between microservices in a distributed architecture. JSON's widespread adoption ensures compatibility and interoperability across different platforms and programming languages, facilitating seamless communication and integration between various components of a system.
- JSON example: 
```json
{
    "name": "Guest Name",
    "email": "email@example.com",
    "room_number": 9999,
    "check_in_date": "00/00/0000",
    "check_out_date": "00/00/0000",
    "num_guests": 0
}

```
# Exception handling
Error handling is managed within the controllers, so in case of any occurrence of an error, the response is a JSON describing the error accompanied by a "Bad Request" status. This process ensures clear communication between the server and the client, making problem identification and resolution more efficient.
- JSON:
```JSON
 {
  "error": {
    "code": 500,
    "message": "Message error"
  }
}
```
# Performance
Using Rust as the language for creating microservices brings significant benefits. Rust is renowned for its safety and speed, offering robust memory safety features that prevent common programming errors like null pointer dereferencing and buffer overflows. This ensures the reliability and stability of microservices even under heavy loads. Additionally, Rust's performance is exceptional, making it well-suited for high-performance applications. Another advantage is Cargo, Rust's package manager, which is multi-platform and simplifies dependency management, allowing for efficient development and deployment of microservices across different environments.
# Licence 
he Hostel By Pass project uses the MIT License, an open and permissive software license that provides developers with great freedom to use, modify, and distribute the source code


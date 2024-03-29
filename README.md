## Directory explanation
- `main.rs`: Entry point for your application.
- `lib.rs`: A module file that allows you to organize your code into multiple files.
- `routes/`: Contains route configurations.
- `handlers/`: Handles HTTP requests, interacts with services.
- `models/`: Defines your data models.
- `schemas/`: Defines your user request schema.
- `services/`: Contains business logic.
- `repositories/`: Manages database interactions.
- `database.rs`: Sets up the database connection using SQLx.
- `utils.rs`: Contains utility functions.

## New API step
1. create new handler in `src/handlers/`
2. create new service in `src/services/`
3. create new repository in `src/repository/`
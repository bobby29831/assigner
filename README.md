# Canvas CLI

### Startup & Setup
There are a few things that you will need in order to start using canvas-cli, and those things are:
- the base url for the canvas instance you are wanting to use (ex. `https://example.instructure.com/`)
- an access token provided by canvas (for testing purposes)
- the built binary/executable from building this project (coming in releases SoonTM)

Upon executing the first command using the file, the program will prompt you for the two details above. Upon entering
those details, there will be a file created in the directory where the file was run called `canvas-cli-config.toml` where
those details will be stored for persistence.

After that, you should be able to use `canvas-cli` as you wish.
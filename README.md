# Aviso
Este projeto vai ser privado durante minha inatividade e não vou ter certeza se isso vai virar open-source futuramente. É isso :^).


## Post Interaction
A small code for receiving Discord interactions, prepared for libraries that receive via WebSocket. What's up? curious to work with this cute little code?

## Download
You can make a **git clone** or directly download **Code** > **Download Zip** and then unzip the file and do the process down here.

## How it works?
- 1 - First you need to have Go Language installed on your machine before you can start us.
- 2 - Second, now we need to configure... Okay, how? Go to "`configuration.yaml.example`" and then rename it to `configuration.yml`. Now we can configure and then run the application
- 3 - Third, we only need to run this command `go run server.go`


### Only endpoint released
In the Discord application portal you will need to put `http://ip:port/interaction` like this and then confirm to make sure the result worked.
- **Remembering if the button persists means the encryption failed or entered the wrong public key.**

**What is the meaning of the status code?**
- 500 - It appears that public key has not been placed or has been removed from the configuration.
- 403 - There is something wrong with the public key
- 401 = Request that was sent has an invalid signature.
- 200 = OK


## Libraries:
**[Javascript](https://github.com/NavyCake/post-interaction-js)**

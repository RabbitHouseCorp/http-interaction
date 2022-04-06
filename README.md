# Http Interaction
A small code for receiving Discord interactions, prepared for libraries that receive via WebSocket. What's up? curious to work with this cute little code?




### How to configure API? 
First thing before doing this process we have to check if there are programs installed on your computer or in the environment that is prepared to deploy this API.

**Windows**
 - Git **(https://git-scm.com/)**
 - Need to have the resources installed on the computer that is Visual Studio
 - Rust Lang **(https://www.rust-lang.org/tools/install)**
 - Intellij IDEA **(https://www.jetbrains.com/pt-br/idea/)** or Visual Studio Code **(https://code.visualstudio.com/)**
   - For development environment or you can use it to modify .env 
 - Rename the file named **.env-example** to **.env**
 

**Linux**
 - Git **(https://git-scm.com/)**
 - Build Essential **(sudo apt-get install build-essential */* sudo apt install build-essential)**
   - It is necessary to install this package on linux because cargo build/run uses it to build the application in order to run the code. 
 - Rust Lang **(https://www.rust-lang.org/tools/install)**
 - Rename the file named **.env-example** to **.env**

### Environment
```yaml
# You can add two keys in the API for example: PUBLICK_KEY= {1} {2}
# Need to add space because the code cuts the braces with space (.split(" "))
#
PUBLIC_KEY={public_key}

# You can add two id bot in the API for example: BOTS_DISCORD= {1} {2}
# Need to add space because the code cuts the braces with space (.split(" "))
#
# 
#
BOTS_DISCORD=

# Need to generate a very strong key if you add API in subdomain or target API to main domain. If you go directly to Discord, you don't need to use a key.
# Recommendations for generating a strong key is to use JWT (Soon there will be support for JWT for strong authentication in the websocket API) or create a giant Base64.
#
KEY_SECRET=
```



### How to run application?
When you finish doing this process above you can run `cargo run` or `cargo build` which will build the application in `bin` or `exe` in the `/target/debug` folder and it will be `httpinteraction`. 



### API access 
When running `cargo run` there will be no log but it will be running `http://127.0.0.1:8080` 


#### What is the endpoint for Discord to access? 
 - The endpoint is `/interaction` 

#### What is the endpoint for me to connect to the API? 
 - The endpoint is `/ws_interaction` 
 - There is header needed to access API which are: 
   - `Identification-Id` -> Bot ID
   - `Secret` -> KEY_SECRET
   - `Public-Key` -> PUBLIC-KEY
   - `Shard-In` -> Shard-In (It's not needed currently, it's still in development you can only initiate just one connection.)
   - `Shard-Total` -> Shard-Total (It's not needed currently, it's still in development you can only initiate just one connection.)

#### Why when I start another connection in the API it closes the previous one?
 - This means that the API currently only supports 1 connection on that ID only. 

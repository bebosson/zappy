Zappy is a 42's School project who aim to create a multiplayer video game. \
AIs play with each others, and the rendering is external. 

The game consists of various components: \
• A server with the current map, resources, it will take care of all the timing in the
game and is also the judge of the game (it will enforce the rules) \
• One or more clients have the ability to connect to the server, with each client
assuming the role of a player. The players are then assigned to teams. \
• A graphic client that will connect to the server to show what is currently happening  

We are currently implementing all this in Rust

Here is the architecture of our server (in french),
we're using bevy 0.12 for the graphical part

![image](https://github.com/bebosson/zappy/assets/45608547/face74cd-9260-4fdc-aa6a-ee60c9f7a917)




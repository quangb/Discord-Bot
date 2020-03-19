
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType,
};
use std::fs;
use rand::prelude::*;


const TOKEN: &str = "";

struct Handler;

impl EventHandler for Handler {

    fn message(&self, ctx: Context, msg: Message) {
        //init vector that hold all file path(String) in folder "folder_name"
        //then use get_random_path function to select a random element
        let mut vector_file: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir("./folder_name"){
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Ok(s) = entry.file_name().into_string(){
                        vector_file.push("./folder_name/".to_owned() + &s);
                    }
                }
            }
        }
        //
        if msg.content == "!command_name" {
            let mut s: String = get_random_path(&vector_file);
            let f_img = fs::File::open(&s).unwrap();
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.add_file(AttachmentType::File{file: &f_img, filename: s});
                m
            });
        }
    }


    
    //print to console the name of the bot
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {

    //create bot
    let mut client = Client::new(&TOKEN, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

fn get_random_path(v :&Vec<String>) -> String{
    let mut rng = thread_rng();
    let index = rng.gen_range(0, v.len());
    v[index].clone()
}

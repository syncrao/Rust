use chapter17::*;

fn main() {
   let mut post = Post::new();

   post.add_text("Hello post added");
   post.request_review();
   post.approve();

   println!("Post: {}", post.content());

}

use chapter17::*;

fn main() {
   let mut post = Post::new();

   post.add_text("Hello post added");
   // post.request_review();
   // post.approve();

   let post = post.request_review();
   let post = post.approve();


   println!("Post: {}", post.content());

}

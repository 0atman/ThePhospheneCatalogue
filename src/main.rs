use color_eyre::Report;
#[allow(clippy::wildcard_imports)]
use html_node::{
    typed::{elements::*, html},
    Node,
};

fn template(inner: Node) -> Node {
    html! {

        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
                <link rel="icon" href="favicon.png"/>
                <script src="https://cdn.tailwindcss.com"></script>

            <script>
                tailwind.config = {
                    theme: {
                        container: {
                            center: true,
                        },
                        fontFamily: {
                            "mono": "courier, monospace",
                        }
                    }
                }
            </script>

            <meta charset="utf-8"/>
            <meta name="description" content="An urban fantasy podcast of tape recordings by the curator of a secretive london-based art auction house."/>

            <meta content="width=device-width, initial-scale=1" name="viewport"/>
            <title>"The Phosphene Catalogue Podcast"</title>

            </head>

                <body class="bg-black text-white font-mono text-sm md:text-2xl">

                    <nav class="flex items-center justify-between flex-wrap bg-black-500 p-6">
                        <div class="flex items-center flex-shrink-0 text-white mr-6">
                            <span class="font-semibold text-xl tracking-tight">The Phosphene Catalogue</span>
                        </div>
                        <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                            <div class="text-xl lg:flex-grow">
                                <a href="https://www.spreaker.com/show/the-phosphene-catalogue" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Listen
                                </a>
                                <a href="https://open.spotify.com/show/5XPmpYIlK2nYOfINCzlYUu?si=00798415bfff4b9e" class="blocklg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Spotify
                                </a>
                                <a href="https://www.spreaker.com/show/6029902/episodes/feed" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    RSS
                                </a>
                                <a href="https://masto.namtao.com/@PhospheneCatalogue" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Mastodon
                                </a>
                                <a href="https://discord.gg/mCY2bBmDKZ" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Discord
                                </a>
                                <a href="https://www.teepublic.com/user/the-phosphene-catalogue" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Store
                                </a>
                                <a href="https://soundcloud.com/namtao/sets/the-phosphene-catalogue-music" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Music
                                </a>

                                <a href="https://www.patreon.com/PhospheneCatalogue" class="block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Patreon
                                </a>
                            </div>
                        </div>
                    </nav>

                    <div class="border-black border-8 container mx-auto">

                        <div class="flex w-full justify-center">
                            <img class="w-1/2" alt="A photo of an art catelogue cover" src="logo.png" />
                        </div>

                            <br/>
                            <br/>
                            {inner}

                    </div>

            </body>
        </html>
    }
}

/// NOTE: the widget requires https to load
fn widget() -> Node {
    html_node::html! {
    <iframe width="560" height="315" src="https://www.youtube.com/embed/y3wVvDbJHrU?si=19zRXxXpwnAGuB0A" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
        }
}

fn index() -> Node {
    template(html! {
          <div class="slogan"> "We see light where others see only darkness." </div>

          <br/>
          <br/>
        <div class="justify">
          "An urban fantasy podcast of tape recordings by the curator of a secretive London-based art auction house."
          <br/>
          <br/>
          The Phosphene Catalogue is a 1970s mail-order catalogue, specialising in those items that cannot be sold at other auction houses: Paintings of lost origin, statues that are "too grotesque" for public display, and books better left unread...
          </div>
      <div class="narrow">
      </div>
    <br/>
    <br/>

        <b><a href="https://www.youtube.com/watch?v=y3wVvDbJHrU">"Listen to the first issue here"</a></b>
        <br/>
        { widget() }
        <br/>
        <br/>
    })
}

type Router<'a> = Vec<(&'a str, fn() -> Node)>;

fn build(pages: Router) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun().to_string())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![("docs/index.html", index)]);
    println!("Built site OK!");
    Ok(())
}

use seed::{prelude::*, *};
use wasm_bindgen::prelude::*;
use web_sys::{File, HtmlInputElement};
use linkedin_api_rs::prelude::{RedirectURL, Token, Config, LoginResponse};
use serde::Serialize;
use serde::Deserialize;
use linkedin_api_rs::client::Client;
use linkedin_api_rs::http_clinent::errors::ClientErr;


// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {

    let mut testing  = url.search().to_string() ;
    let mut login_respose;
    if testing.is_empty() {
        login_respose  = None;
    }     else {
           login_respose  = Some(LoginResponse::extract_user_tokens(testing));
    }
 orders.send_msg(
     Msg::ConfigFetched(
     Config::new(
         "https://www.linkedin.com/oauth/v2/authorization".to_string(),
         "78r2gfrobwen7a".to_string(),
         "78r2gfrobwen7a".to_string(),
         "http://localhost:8001/".to_string())
 ));

    Model {
        redirect_url: RedirectURL::default(),
        login_response  :login_respose ,
        user_tokens: None,
       token_exchange_url : None,
       // accounts: None,

    }

}

#[derive(Default, Serialize,Deserialize)]
struct TestPostData {
    author: String,
    lifecycleState:String,
   // specificContent: SpecificContent,
    //visibility: String,
}
struct  SpecificContent{
    shareCommentary:       String,
    shareMediaCategory: String,
   // media:Media
}

struct  Media{
    status:String,
    description: String,
    //media:
}
#[derive(Default, Serialize)]
struct  Test{
    code:String
}
// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.

#[derive(Default)]
pub struct Model {

    redirect_url: RedirectURL,
     login_response : Option<LoginResponse>,
     user_tokens: Option<Token>,
     token_exchange_url : Option<RedirectURL>
  //  accounts: Option<Data<Accounts>>,
    //switch_account_to: String,
   // facebook: facebook::Model,
   // instagram: instagram::Model,
}


// ------ ------
//    Update
// ------ ------

enum Msg {
    ConfigFetched(Config),
    InitTokenConfig,
   GetTokenConfig(Config) ,
   GetToken,
   GetTokenResponse(Token) ,
   TestPost,
   TestPostResponse(fetch::Result<Token>),
   // GetAccount,
   // GetAccountSuccess(Data<Accounts>),
    // every error should user this
    ResponseFailed(ClientErr),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
       Msg::ConfigFetched(config) =>{
           log!(config);
           model.redirect_url = RedirectURL::new(config)
               .add_response_type("code")
               .add_scope(&["r_liteprofile".to_string(),"r_emailaddress".to_string(),"w_member_social".to_string()]).add_full_url();
           log!(model.redirect_url.full_url);
       }
        Msg::ConfigFetched(fetch_error) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client_id and redirect_uri", fetch_error),

        Msg::InitTokenConfig =>{

            orders.send_msg(
                Msg::GetTokenConfig(

                    Config::new(
                        "https://www.linkedin.com/oauth/v2/accessToken".to_string(),
                        "78r2gfrobwen7a".to_string(),
                        "78r2gfrobwen7a".to_string(),
                        "http://localhost:8001/".to_string())
                ))     ;
        }

        Msg::GetTokenConfig(config)  => {
            if let Some(access_code) = &model.login_response{
                let code = &access_code.code;
                   model.token_exchange_url =  Some( RedirectURL::new(config).token_exchange_url(code.to_string()) );
            }
            log!(model.token_exchange_url )
    }
        Msg::GetTokenConfig(config_eror)         => {

        }

        Msg::GetToken =>{
              if let Some(token_url) = &model.token_exchange_url {
                  let build_url =   token_url.full_url.clone();
                  orders.skip().perform_cmd(async move {
                       Client::accss_token(build_url.to_string().clone())
                          .get()
                          .await
                          .map_or_else(Msg::ResponseFailed, Msg::GetTokenResponse)
                  });
              }

        }
               Msg:: GetTokenResponse(resp) => {
                   log!(resp)
               }

        Msg::TestPost =>{


            orders.perform_cmd(async {
              Msg::TestPostResponse(send_message().await)});

        }

    Msg::TestPostResponse(resp) => {

    }

        // all errro should user this, except the eeror neededs to be analyzed and do something about it
        Msg::ResponseFailed(resp) => {
            log!(resp)
        }
    }
}

// this was a testing method
async fn send_message() -> fetch::Result<Token> {
    let url =  "https://api.linkedin.com/v2/ugcPosts";
     let bearHeader =Header::custom("X-Restli-Protocol-Version","2.0.0");
        let authorizaion = Header::authorization("Bearer: Token");
        Request::new(url)
        .method(Method::Post).header(bearHeader).header(authorizaion)
        .json(&TestPostData{author:"urrur".to_string(),lifecycleState:"rrr".to_string()})?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        h1![
            " Welcome to the example on Linkedin API",
            style! {
               St:: TextAlign => "center",
            },
        ],

       div![
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            a![
                attrs! {
                    At::Href => model.redirect_url.get_full_url()
                },
                button![
                    img![
                        attrs! {
                            At::Src => "images/sign_in_small.png",
                          // At::Src => "src/login_button.png", // attribute <a href="https://www.freeiconspng.com/img/18026">Facebook login button png</a>
                        },
                        style! {
                         St::PaddingTop => px(3),
                         St:: MarginLeft => px(7)
                        },
                    ],
                ]
            ],

            style! {
                St::Height => "50px"
            },
        ],


       div![
            style! [
               St:: Display => "flex",
               St:: JustifyContent => "center",
               St:: MarginTop =>  "20px"
            ],
            style! {
                St::Height => "50px"
            },
            button![
                "Build token url !",
                ev(Ev::Click, |_| { Msg::InitTokenConfig }),
                attrs! {
                    At:: Disabled => model.login_response.is_none().as_at_value()
                }
            ],
            button![
                "Get token  !",
                ev(Ev::Click, |_| { Msg::GetToken }),
                attrs! {                                                            
                    At:: Disabled => model.token_exchange_url.is_none().as_at_value()
                }
            ],
             button![
                "Test post   !",
                ev(Ev::Click, |_| { Msg::TestPost }),
                attrs! {
                    At:: Disabled => model.token_exchange_url.is_none().as_at_value()
                }
            ],
        ],
    ]
}


// ed::browser::dom::event_handler
// pub fn ev<Ms: 'static, MsU: 'static>(trigger: impl Into<Ev>, handler: impl
// FnOnce(web_sys::Event) -> MsU + 'static + Clone) -> EventHandler<Ms>

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

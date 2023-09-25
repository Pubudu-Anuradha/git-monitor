mod util;

use crate::{get_prisma_connection, prisma};
use actix_web::web::{self, Json};

pub fn config_scope() -> actix_web::Scope {
  web::scope("config")
    .route("", web::get().to(get_config))
    .route("", web::put().to(update_config))
}

async fn update_config() -> Json<Option<prisma::git_config::Data>> {
  println!("Updating config");
  let config = git2::Config::open_default().unwrap();
  let res = get_prisma_connection()
    .await
    .git_config()
    .delete_many(vec![])
    .exec()
    .await;

  if res.is_ok() {
    let mut entries = Vec::new();

    let _ = config.entries(None).unwrap().for_each(|entry| {
      let name = entry.name().unwrap();
      let value = entry.value().unwrap();
      entries.push((name.to_string(), value.to_string()));
    });

    let data = get_prisma_connection()
      .await
      .git_config()
      .create(vec![])
      .exec()
      .await
      .unwrap();

    for (name, value) in entries {
      let _ = get_prisma_connection()
        .await
        .git_config_entry()
        .upsert(
          prisma::git_config_entry::UniqueWhereParam::NameEquals(name.clone()),
          (
            name,
            value,
            prisma::git_config::UniqueWhereParam::IdEquals(data.id.clone()),
            vec![],
          ),
          vec![],
        )
        .exec()
        .await;
    }
  }
  get_config().await
}

async fn get_config() -> Json<Option<prisma::git_config::Data>> {
  Json(
    get_prisma_connection()
      .await
      .git_config()
      .find_first(vec![])
      .with(prisma::git_config::entries::fetch(vec![]))
      .exec()
      .await
      .unwrap(),
  )
}

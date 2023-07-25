#[allow(dead_code, unused)]
pub mod prisma;
pub mod services;

pub async fn get_prisma_connection() -> prisma::PrismaClient {
  prisma::PrismaClient::_builder().build().await.unwrap()
}

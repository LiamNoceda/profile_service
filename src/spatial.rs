use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub async fn get_spatial_handler(State(ctx): State<PgPool>, claims: Claims,) -> Result<Json<>>, StatusCode> {
    // Функция которая будет отправлять основные данные с бд на фронт
    // Которые были добавленны с регистрации (username, phone, email, castom_id)
    // А также последующие данные после изменений (name, fullname, прошлые данные с регистрации, и т.д.)

    /*
    Пример запроса
        бек хендлер принимает запрос с фронта, который содержит кастомный идентификатор пользователя (custom_id).
        Затем он выполняет SQL-запрос к базе данных, чтобы получить данные пользователя, связанные с этим идентификатором. Если данные найдены, они возвращаются в формате JSON.
        Если данные не найдены, возвращается статус 404 Not Found.
        для логирования ошибок используется логгер, который записывает ошибки в файл или выводит их в консоль.
        tracing::error!("Error fetching user data: {:?}", e);

        1 Получает get запрос на получения данных пользователя по кастомному идентификатору (custom_id).
        2 Выполняет SQL запрос к бд для получения данных пользователя по кастомному идентификатору (custom_id).
        3 Данные найдены, возврашает в формате JSOM
        4 Или возвращает статус 404
    */

    payload.validate()?;

    let get_pool = query_as!(
        "SELECT username FROM users WHERE castom_id = $1",
        &claims.sub
    )
    .fetch_optional(&ctx)
    .await?
    .or_or_else(|| AppError::Conflict("User spatial not found in system"))?;

    Ok(Json(get_pool))
}

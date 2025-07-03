use backend::repositories::ProductRepository;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let repo = ProductRepository::new();

    loop {
        println!("1. Добавить товар");
        println!("2. Просмотреть товары");
        println!("3. Выход");
        

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_product(&repo),
            "2" => list_products(&repo),
            "3" => break,
            _ => println!("Неверный выбор"),
        }
    }
}

fn add_product(repo: &ProductRepository) {
    println!("Введите название товара:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Введите цену:");
    let mut price = String::new();
    std::io::stdin().read_line(&mut price).unwrap();
    let price: f64 = price.trim().parse().unwrap();

    let product = backend::models::Product {
        id: Uuid::new_v4(),
        name: name.trim().to_string(),
        price,
        description: String::new(),
        category: String::new(),
        stock: 0,
        image_url: String::new(),
        is_hidden: false,
        encrypted_details: String::new(),
    };

    match repo.add_product(product) {
        Ok(_) => println!("Товар успешно добавлен!"),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn list_products(repo: &ProductRepository) {
    println!("Список товаров:");
    match repo.list_products() {
        Ok(products) => {
            for product in products {
                println!("- {}: {} руб. (ID: {})",
                        product.name, product.price, product.id);
            }
        }
        Err(e) => println!("Ошибка при получении списка товаров: {}", e),
    }
}
pub fn with_vectors() {
    // Вектор
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    println!("Vector: {:?}", v);
    v[0] = 456;
    println!("Vector: {}", v[0]);

    let v2 = vec![1, 2, 3, 4, 5];

    match v2.get(20) {
        Some(value) => println!("Vector el: {}", value),
        None => println!("Vector el is empty"),
    }

    let mut v3 = vec![100, 200, 300];
    v3.pop(); // удалить первый элементы
    v3.push(345); // добавить элемент в конец
    v3.remove(1); // удалить элемент по индексу
    for value in &v3 {
        println!("El: {}", value);
    }
}
# bhft-exchange
Тестовое задание для BHFT. Простая биржа.

# Запуск
1. `git clone https://github.com/flisoch/bhft-exchange.git && cd bhft-exchange/exchange`
2. `cargo run`

## Запуск тестов
1. `cargo test`

# Алгоритм
1. Добавляем по очереди заявки
2. если очередная матчится по цене с уже имеющимися, то они или закрывают друг друга, или одна закрывает другую и одна остаётся

# Комментарии
1. Не верно считает баланс при таком сценарии: допустим уже есть заявка на покупку актива за $10. Если мы добавляем заявку на продажу актива за $9, то 
заявки сматчатся, но цена будет не 10, а 9
2. голые unwrap'ы, постоянные borrow'ы и прочее выглядит плохо, но пока не привык и не знаю, как принято

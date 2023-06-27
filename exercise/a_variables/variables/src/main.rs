/*
Lessons
1. 상수는 SCREAMING_SNAKE_CASE 사용
- 타입 어노테이션 필수
- 값은 컴파일 시점에 결정되는 constant expression이여야 함
2. Rust는 컴파일 타임에 memory safety를 개런티함
- 변수는 실사용 전에 이니셜라이즈 되어야함
- if true와 같은 코드 같이 특정 조건에 변수를 사용한다해도 conditional evaludation은 compile time이 아닌 runtime에 핸들되기 때문에 컴파일러는 해당 부분을 개런티 할 수 없음
- 하지만 if true {} else {}와 같은 코드는 컴파일러에서 intialized 된다고 개런티 할수 있음 
 */


/*
Part 1

fn main() {
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}
 */


fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missles left", missiles);
}

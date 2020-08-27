//引用
use mylib::factory::produce_refrigerator;
use mylib::factory::produce_washing_machine;
//或者是
use mylib::factory::*;
fn main() {
    //绝对路径
    mylib::factory::produce_refrigerator::produce_re();
    mylib::factory::produce_washing_machine::produce_wm();
    //使用 use
    produce_refrigerator::produce_re();
    produce_washing_machine::produce_wm();
}

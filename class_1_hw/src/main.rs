// bai 1
fn check_sub_array(A:&[usize],B:&[usize]) {
    let mut i:usize=0;
    let mut j:usize=0;
    while i < A.len() && j< B.len() {
        if A[i]==B[j] {
            i+=1;
            j+=1;
            if j == B.len() {
                println!("yes");
                return;
            }
        }
        else {
            i = i-j+1;
            j=0;
        }
    }
    println!("nah");
}

// bai 2
fn count_occurences(reference: String, check_str: &str){
    let res = reference.matches(&check_str).count();
    println!("{}",res);
}

// huong lam bai nang cao
// fn count2(input_str:String){
//     let re:String=format!(r"()",input_str);
//     let re:Regex=Regex::new(re:re.as_str()).unwrap();
//     let count:usize=re.captures_iter(text:SENTENCE).count();
// }

fn main() {

    let a=[1,2,3];
    let b=[1,2];
    // check_sub_array(&a,&b);

    let re = std::fs::read_to_string("test.txt").unwrap();

    println!("Your input: ");
    let mut check_str: String = String::new();
    std::io::stdin().read_line(&mut check_str);

    count_occurences(re,&check_str);

}


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


fn count_occurences(reference: &str, check_str: &str){
    let res = reference.matches(&check_str).count();
    println!("{}",res);
}


fn main() {

    let a=[1,2,3];
    let b=[1,2];
    // check_sub_array(&a,&b);

    // let mut file= File::open("test.txt");
    // let mut re = String::new();
    // file.read_to_string(&mut re).expect("err");

    let re = std::fs::read_to_string("test.txt").unwrap();

    println!("Your input: ");
    let mut check_str: String = String::new();
    std::io::stdin().read_line(&mut check_str);

    count_occurences(&re,&check_str);

}
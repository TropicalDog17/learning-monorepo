pub enum WithNullPtrOptimization{
    A,
    B(String),
}

pub enum WithoutNullPtrOptimization{
    A,
    B(u32),
}

fn main()  {
    println!("{} {}", std::mem::size_of::<WithNullPtrOptimization>(), std::mem::size_of::<String>()); 
    println!("{} {}", std::mem::size_of::<WithoutNullPtrOptimization>(), std::mem::size_of::<u32>()); // 8 4
}
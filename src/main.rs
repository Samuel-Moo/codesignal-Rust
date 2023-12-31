#![allow(warnings)]

fn add(param1: i32, param2: i32) -> i32 {
    param1 + param2
}


fn century_from_year(year: i32) -> i32 {
    let mut century = 1;
    
    while century*100 < year{
        century += 1;
    }
    
    century
    
}

fn checkPalindrome(inputString: String) -> bool {
    let newString: String = inputString.chars().rev().collect();
    
    let mut res: bool = true;
    
    if newString != inputString {
        res = false
    } 
    
    res
    
}

fn adjacentElementsProduct(inputArray: Vec<i32>) -> i32 {

    let mut largest = i32::MIN;

    for i in 0..inputArray.len() - 1 {
        
        let mut memo = inputArray[i] * inputArray[i + 1];
        
        if memo > largest {
            largest = memo;
        }
    }
    
    largest
}

fn shapeArea(n: i32) -> i32 {
    n.pow(2) + (n-1).pow(2) 
}

fn MakeArrayConsecutive2(statues: Vec<i32>) -> i32 {
    let mut newStat: Vec<i32> = statues.clone();
    newStat.sort();
    let mut solved: i32 = 0;
    for i in 0..newStat.len() - 1 {
        if newStat[i + 1] - newStat[i] > 1 {
            solved += newStat[i + 1] - newStat[i] - 1; 
        }
    }
    solved
}


fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut total_sum: i32 = 0;
    for (row_id, row) in matrix.iter().enumerate() {
        for (element_id, &(mut element)) in row.iter().enumerate() {
            if row_id > 0 && matrix[row_id - 1][element_id] == 0 {
                element = 0;
            }
                        if row_id > 1 && matrix[row_id - 2][element_id] == 0 {
                element = 0;
            }
                        if row_id > 2 && matrix[row_id - 3][element_id] == 0 {
                element = 0;
            }
            if element != 0 {
                total_sum += element;
            }  
        }
    } 
    total_sum
}


fn main() {
    //Execute Stuff
    let function_to_test = century_from_year(1898); 
    
    println!("Result: {}", function_to_test);
}



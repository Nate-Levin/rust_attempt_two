#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use serde_derive::{Deserialize, Serialize};
use csv::Reader;
use std::error::Error;
use std::collections::HashMap;
use std::str::FromStr;
use std::process;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AnzFile {
    pub details: String,
    pub particulars: String,
    pub code: String,
    pub reference: String,
    pub amount: String,
    pub date: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QbFile {
    pub name: String,
    pub split: String,
    pub date: String,
    pub amount: String
}
#[derive(Debug)]
pub struct AnzErrorMessage{
    pub amount: String,
    pub frequency: usize,
    pub qb_frequency: usize,
    pub qb_dates: Vec<String>,
    pub qb_names: Vec<String>,
    pub dates: Vec<String>,
    pub details: Vec<String>,
    pub particulars: Vec<String>,
    pub error_message: String 
}
#[derive(Debug)]
pub struct QbErrorMessage{
    pub amount: String,
    pub dates: Vec<String>,
    pub names: Vec<String>,
    pub anz_frequency: String,
    pub anz_dates: Vec<String>,
    pub anz_names: Vec<String>,
    pub error_message: String 
}
#[derive(Debug)]
pub struct DoesntExistMessage{
    pub amount: String,
    pub dates: Vec<String>,
    pub names: Vec<String>,
    pub error_message: String
}

impl AsRef<String> for AnzFile {
    fn as_ref(&self) -> &String {
        &self.amount
    }
    fn as_ref_date(&self) -> &String{
        &self.date
    }
}

impl AsRef<String> for QbFile {
    fn as_ref(&self) -> &String {
        &self.amount
    }
    fn as_ref_date(&self) -> &String{
        &self.date
    }
}

pub trait AsRef<String>{
    fn as_ref(&self) -> &String;
    fn as_ref_date(&self) -> &String;
}


fn main(){

    let mut anz_struct_data: Vec<AnzFile> = Vec::new();
    let mut qb_struct_data: Vec<QbFile> = Vec::new();

    let anz_file: String = "src/one_year_anz.csv".to_owned();
    let qb_file: String = "src/one_year_qb.csv".to_owned();

    match read_csv_return_struct(&anz_file){
        Err(e) =>{
            eprintln!("{}", e);
        }
        Ok(v)=>{
            anz_struct_data = v;
        }
    }

    match read_csv_return_struct(&qb_file){
        Err(e) =>{
            eprintln!("{}", e);
        }
        Ok(v)=>{
            qb_struct_data = v;
        }
    }
    let mut anz_hash: HashMap<String, Vec<AnzFile>> = HashMap::new();
    let mut anz_hash_count: HashMap<String, i32> = HashMap::new();
    let mut qb_hash: HashMap<String, Vec<QbFile>> = HashMap::new();
    let mut qb_hash_count: HashMap<String, i32> = HashMap::new();
    let mut anz_values_to_remove: Vec<String> = Vec::new();
    let mut qb_values_to_remove: Vec<String> = Vec::new();

    check_and_prune_structs(&mut anz_struct_data, &mut qb_struct_data);
    check_and_return_hash(anz_struct_data.clone(), &mut anz_hash, &mut anz_hash_count);
    check_and_return_hash(qb_struct_data.clone(), &mut qb_hash, &mut qb_hash_count);
    
    for (k1, v1) in qb_hash_count.iter(){
        // matches with the leading 0 removed
        let without_trailing_zero = remove_trailing_zeros(k1.clone().to_string(), 0);
        
        // matches the literal
        (anz_values_to_remove, qb_values_to_remove) = get_matching_values(anz_hash_count.clone(), anz_values_to_remove, qb_values_to_remove, k1.to_string(), k1.to_string(), v1);
        (anz_values_to_remove, qb_values_to_remove) = get_matching_values(anz_hash_count.clone(), anz_values_to_remove, qb_values_to_remove, without_trailing_zero.clone(), k1.to_string(), v1);
        
    }

    (anz_hash_count, qb_hash_count) = remove_matching_values_from_counter(anz_values_to_remove, anz_hash_count, qb_hash_count);
    (anz_hash_count, qb_hash_count) = remove_matching_values_from_counter(qb_values_to_remove, anz_hash_count, qb_hash_count);

    // At this point we only have the values that don't exist, entered incorrectly,
    // or don't show up as many times as expected. 

    // first lets look, if we have a corrosponding value that matches we can figure out
    // who we needs to investigate. 
    let mut doesnt_exist_in_anz:Vec<String> = Vec::new();
    let mut doesnt_exist_in_qb:Vec<String> = Vec::new();
    let mut investigate_anz: Vec<String> = Vec::new();
    let mut investigate_qb: Vec<String> = Vec::new();

    for (k1, v1) in &qb_hash_count{
        // to eliminate the fact it still could be decimal issue
        let mut exists = false;
        match anz_hash_count.get(&k1 as &str){
            Some(v) => {
                if v > &v1 {
                    investigate_qb.push(k1.clone());
                }
                else{
                    investigate_anz.push(k1.clone());
                }
                exists = true;
                continue;
            }
            None => (),
        }      
        let mut trailing_zero = remove_trailing_zeros(k1.clone().to_string(), 0);
        match anz_hash_count.get(&trailing_zero as &str){
            Some(v) => {
                if v > &v1 {
                    investigate_qb.push(k1.clone());
                }
                else{
                    investigate_anz.push(k1.clone());
                }
                exists = true;
                continue;
            }
            None => (),
        }
        if exists == false{
            doesnt_exist_in_anz.push(k1.clone());
        }
    }

    for (k1, v1) in &anz_hash_count{
        // to eliminate the fact it still could be decimal issue
        let mut exists = false;
        for (k2,v2) in &qb_hash_count {
            match remove_trailing_zeros(k1.clone().to_string(), 0) == remove_trailing_zeros(k2.clone().to_string(), 0){
                true => {
                    exists = true;
                }
                false => (),
            }
        }
        if ! exists{
            doesnt_exist_in_qb.push(k1.clone());
        }
    }

    // investigate anz has values that appear more often in anz,
    // investigate qb has values that appear more often in qb,
    // doesnt_exist has values that only appear in one or the other
    // No we need to get matches,

    //dealing with no matches
    let mut doesnt_exist_error: Vec<DoesntExistMessage> = Vec::new();
    for i in &doesnt_exist_in_anz{
        match qb_hash.get_key_value(i){
            Some((k,v)) => {
                let mut error_message:String = format!("The value {} exists in QUICKBOOKS but can't be found in ANZ", i.clone());
                let mut dates: Vec<String> = Vec::new(); 
                let mut names: Vec<String> = Vec::new(); 
                for mut x in v.clone() {
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    dates.push(x.clone().date);
                    names.push(x.clone().name);
                }
                let mut temp: DoesntExistMessage = DoesntExistMessage {
                    amount: i.clone(),
                    error_message: error_message,
                    dates: dates,
                    names: names
                };
                doesnt_exist_error.push(temp);
            },
            None => (),
        }
    }

    for i in &doesnt_exist_in_qb{
        match anz_hash.get_key_value(i){
            Some((k,v)) => {
                let mut error_message:String = format!("The value {} exists in ANZ but can't be found in QUICKBOOKS", i.clone());
                let mut dates: Vec<String> = Vec::new(); 
                let mut names: Vec<String> = Vec::new();
                for mut x in v.clone() {
                    dates.push(x.clone().date);
                    names.push(x.clone().details);
                }
                let mut temp: DoesntExistMessage = DoesntExistMessage {
                    amount: i.clone(),
                    error_message: error_message,
                    dates: dates,
                    names: names
                };
                doesnt_exist_error.push(temp);
            },
            None=>(),
        }
    }

    let mut anz_error: Vec<AnzErrorMessage> = Vec::new();
    for i in investigate_anz{
        match qb_hash.get_key_value(&i){
            Some((k,v)) => {
                let mut error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
                let mut dates: Vec<String> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                let mut particulars: Vec<String> = Vec::new();
                let mut details: Vec<String> = Vec::new();
                let mut qb_dates: Vec<String> = Vec::new();
                let mut qb_names: Vec<String> = Vec::new();
                let mut freq: usize = 0;

                for mut x in v.clone(){
                    qb_dates.push(x.clone().date);
                    if x.name == ""{
                        x.name = "--SPLIT--".to_owned();
                    }
                    qb_names.push(x.clone().name);
                }

                let str1: String = remove_trailing_zeros(i.to_string(), 0);
                match anz_hash.get_key_value(&str1){
                    Some((k,v)) => {
                        for mut x in v.clone(){
                            dates.push(x.clone().date);
                            names.push(x.clone().details);
                            particulars.push(x.clone().particulars);
                            freq = v.len();
                        }
                    }
                    None => (),
                }

                let mut anz: AnzErrorMessage = AnzErrorMessage{
                    amount: i.clone(),
                    frequency: freq,
                    qb_frequency: v.clone().len(),
                    qb_dates: qb_dates,
                    qb_names: qb_names,
                    details: names,
                    dates: dates,
                    particulars: particulars,
                    error_message: error_message
                };

                anz_error.push(anz);
            }
            None => (),
        }
    }

    for i in anz_error{
        println!("{} \nthis value appears {} times in ANZ vs {} times in QUICKBOOKS",
        &i.error_message, &i.frequency, &i.qb_frequency);
        println!("\nANZ details are as follows: ");
        println!("\nDates: ");
        for x in &i.dates{
            println!("{}", x);
        }
        println!("\nDetails: ");
        for x in &i.details{
            println!("{}", x);
        }
        println!("\nQUICKBOOKS details are as follows:");
        println!("\nDates: ");
        for x in &i.qb_dates{
            println!("{}", x);
        }
        println!("\nNames: ");
        for x in &i.qb_names{
            println!("{}", x);
        }
        println!("\n\n");
    }
    let mut qb_error: Vec<QbErrorMessage> = Vec::new();
    for i in investigate_qb{
        match anz_hash.get_key_value(&i){
            Some((k,v)) => {
                let mut error_message = format!("The value {} exists more often in QUICKBOOKS than it does in ANZ", &i);
                let mut dates: Vec<String> = Vec::new();
                let mut names: Vec<String> = Vec::new();
                let mut anz_names: Vec<String> = Vec::new();
                let mut anz_dates: Vec<String> = Vec::new();

                for mut x in v.clone(){
                    anz_names.push(x.clone().details);
                    anz_dates.push(x.clone().date);
                }
                match qb_hash.get_key_value(&i){
                    Some((k,v)) => {
                        for mut x in v.clone(){
                            dates.push(x.clone().date);
                            names.push(x.clone().name);
                        }
                    }
                    None => (),
                }
                let mut qb: QbErrorMessage = QbErrorMessage{
                    amount: i.clone(),
                    dates: dates,
                    names: names,
                    anz_frequency: v.len().to_string(),
                    anz_dates: anz_dates, 
                    anz_names: anz_names,
                    error_message: error_message
                };

                qb_error.push(qb);
            }
            None => (),
        }
    }

    for i in doesnt_exist_error{
        println!("{}", &i.error_message);
        println!("\nThese occur on dates:");
        for x in &i.dates{
            println!("{}", x);
        }
        println!("\nBy:");
        for x in &i.names{
            println!("{}", x);
        }
        println!("\n\n");
    }

}

fn check_and_prune_structs<T: AsRef<String> + Clone + std::fmt::Debug,
                           U: AsRef<String> + Clone + std::fmt::Debug>
                          (struct_one: &mut Vec<T>, struct_two: &mut Vec<U>){
    let mut struct_one_index_to_remove: Vec<i32> = Vec::new();                     
    let mut struct_two_index_to_remove: Vec<i32> = Vec::new();

    for (pos, i) in struct_one.iter().enumerate(){
        for (pos2, i2) in struct_two.iter().enumerate(){
            if &i.as_ref() == &i2.as_ref() ||
            (&i.as_ref()).to_string() == remove_trailing_zeros((&i2.as_ref()).to_string(), 0)
            {
                if &i.as_ref_date() == &i2.as_ref_date(){
                    struct_one_index_to_remove.push(pos as i32);
                    struct_two_index_to_remove.push(pos2 
                        as i32);
                }
            }
        }
    }
    let mut struct_one_removed:i32 = 0; 
    let mut struct_two_removed:i32 = 0; 

    for mut i in struct_one_index_to_remove{
        i -= struct_one_removed;
        struct_one.remove(i as usize);
        struct_one_removed += 1;
    }

    struct_two_index_to_remove.sort();
    for mut x in struct_two_index_to_remove{
        if x != 0{
            x -= struct_two_removed;
        }
        struct_two.remove(x as usize);
        struct_two_removed += 1;
    }
}

fn check_and_return_hash<T: AsRef<String> + Clone>
                        (struct_data: Vec<T>, hash_map: &mut HashMap<String, Vec<T>>,
                         hash_counter: &mut HashMap<String, i32>){
    for i in struct_data{ 
        match hash_map.get_mut(&*i.as_ref()){
            Some(v) =>{
                v.push(i.clone());
            }
            None =>{
                hash_map.insert((*i.as_ref().clone()).to_string(), vec![i.clone()]);
            }
        }
        match hash_counter.get_mut(&*i.as_ref()){
            Some(v) =>{
                *v += 1;
            }
            None =>{
                hash_counter.insert((*i.as_ref().clone()).to_string(), 1);
            }
        }
    }
}

fn get_matching_values(comp_hash_count: HashMap<String, i32>, mut values_to_remove: Vec<String>, mut original_values: Vec<String>, 
                       key:String, original_key: String, value: &i32) -> (Vec<String>, Vec<String>){
    
    match comp_hash_count.get(&key as &str){
        Some(v) => {
            if value == v{
                values_to_remove.push(key.clone());
                original_values.push(original_key.clone());
            }
        }
        None => (),
    }
    (values_to_remove, original_values)
}

fn remove_matching_values_from_counter(values_to_remove:Vec<String>, mut hash_counter_one:HashMap<String, i32>,
                                       mut hash_counter_two:HashMap<String, i32>) -> (HashMap<String, i32>, HashMap<String, i32>){

    for i in values_to_remove{
        match hash_counter_one.remove(&i as &str){
            Some(v) => {}
            None => (),
        }
        match hash_counter_two.remove(&i as &str){
            Some(v) => {}
            None => (),
        }
    }
    
    (hash_counter_one, hash_counter_two)
}

fn remove_trailing_zeros(mut number: String, mut iterations: i32) -> String{

    if iterations >= 3{
        return number.to_string();
    } 
    if number.chars().last().unwrap() == '0' || number.chars().last().unwrap() == '.' {
        number.pop();
        number = remove_trailing_zeros(number.clone(), iterations + 1);
    }

    number.to_string()
}

fn read_csv_return_struct<T: for<'de> serde::Deserialize<'de>>(csv_file: &str) -> Result<Vec<T>, Box<dyn Error>>{
    let mut rdr = Reader::from_path(csv_file)
    .expect("COULDNT READ anz_file");
    let iter = rdr.deserialize();
    let mut csv_file_struct: Vec<T> = Vec::new();

    for result in iter{
        csv_file_struct.push(result?);
    }
    Ok(csv_file_struct)
}

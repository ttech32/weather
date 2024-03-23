//Import the necessary libraries
use std::io;
use serde::Deserialize;
use colored::*;
//use reqwest::*;

//I will create a struct to deserialize the json response form the open API
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

//A struct for weather's description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

//A struct for main weather properties
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

//A struct for the wind
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn main() {
    println!("{}", "Welcome to Ashi Weather Station".red().to_uppercase());

    loop{
        //Reading the City
        println!("{}", "Please enter your city".green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Could not accept city");
        let city = city.trim();

        //Reading the Country's Code
        println!("{}", "Please enter your country's code".yellow());
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Could not accept city");
        let country_code = code.trim();

        //Api_Key
        let api_key = "15f7e4d3ae742814bc0321d05bfd52cd";

        //calling the function to fetch weather info
        match get_weather_info(&city, &country_code, &api_key) {
            Ok(response) => display_weather(&response),
            Err(e) => eprintln!("Error: {}", e),
        }

        //Ask if the user wants to continue or not
        println!("{}", "Do you want to continue searching other cities weather, yes/no".green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();


        //An if statement to justify the persons word
        if input == "no" {
            println!("Thank you for using the software have a nice day, you can come back anytime you so wish");
            break;
        }
    }

}

//create your functions here

//create a function to get info from the API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key);

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    
    Ok(response_json)

}

//A function to display result to the screen
fn display_weather(response: &WeatherResponse){
    //Extract the weather info from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let pressure = response.main.pressure;
    let humidity = response.main.humidity;
    let wind_speed = response.wind.speed;

    //Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {}
        > Temperature: {:.1}C
        > Humidity: {:.1}%
        > Pressure: {:.1}hpa
        > speed: {:.1}m/s",
        response.name,
        description,
        temperature,
        humidity,
        pressure,
        wind_speed,
    );
    println!("{}", weather_text);
}
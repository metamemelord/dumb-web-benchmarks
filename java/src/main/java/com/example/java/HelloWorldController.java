package com.example.java;

import org.springframework.web.bind.annotation.GetMapping;  
import org.springframework.web.bind.annotation.RestController;  
@RestController  
public class HelloWorldController {  
@GetMapping(path="/", produces = "application/json")  
public String hello()   
{  
return "OK";  
}  
}
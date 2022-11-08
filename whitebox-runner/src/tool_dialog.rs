use serde_json::Value;
use crate::MyApp;
use crate::toggle;
// use duct;
use std::f32;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::process::{Command, Stdio};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::sync::mpsc::channel;

fn parse_parameters(parameters: &Value) -> Vec<ToolParameter> {
    let mut ret = vec![];
    let empty: Vec<Value> = vec![];
    let tool_parameters = parameters["parameters"].as_array().unwrap_or(&empty);
    for j in 0..tool_parameters.len() {
        let name = tool_parameters[j]["name"].as_str().unwrap_or("").to_string();
        let empty_arr: Vec<Value> = vec![];
        let flags: Vec<String> = tool_parameters[j]["flags"]
            .as_array()
            .unwrap_or(&empty_arr)
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_owned())
            .collect();
        let description = tool_parameters[j]["description"].as_str().unwrap_or("").to_string();
        let default_value = if tool_parameters[j]["default_value"].is_string() {
            Some(tool_parameters[j]["default_value"].as_str().unwrap_or("").to_string())
        } else {
            None
        };
        let optional = tool_parameters[j]["optional"].as_bool().unwrap_or(false);

        let mut str_vec_value: Vec<String> = vec![];
        let mut str_value = "".to_string();
        let mut bool_value = false;
        // let mut int_value = 0usize;
        // let mut float_value = f32::NAN;
        let mut file_type = ParameterFileType::Any;
        let mut geometry_type = VectorGeometryType::Any;

        let parameter_type = tool_parameters[j]["parameter_type"].clone();
        let pt = if parameter_type.is_string() {
            let s = parameter_type.as_str().unwrap_or("").to_lowercase();
            if s == "boolean" {
                if default_value.is_some() {
                    bool_value = default_value.clone().unwrap().trim().to_lowercase().parse().unwrap_or(false);
                }
                ParameterType::Boolean
            } else if s == "float" {
                if default_value.is_some() {
                    // float_value = default_value.clone().unwrap().trim().to_lowercase().parse().unwrap_or(0f32);
                    str_value = default_value.clone().unwrap().trim().to_owned();
                }
                ParameterType::Float
            } else if s == "integer" {
                if default_value.is_some() {
                    // int_value = default_value.clone().unwrap().trim().to_lowercase().parse().unwrap_or(0usize);
                    str_value = default_value.clone().unwrap().trim().to_owned();
                }
                ParameterType::Integer
            } else if s == "string" {
                if default_value.is_some() {
                    str_value = default_value.clone().unwrap();
                }
                ParameterType::String
            } else if s == "directory" {
                if default_value.is_some() {
                    str_value = default_value.clone().unwrap();
                }
                ParameterType::Directory
            } else if s == "stringornumber" {
                if default_value.is_some() {
                    str_value = default_value.clone().unwrap();
                }
                ParameterType::StringOrNumber
            } else {
                println!("Unknown String: {:?}", parameter_type);
                ParameterType::String
            }
        } else if parameter_type.is_object() {
            if !parameter_type["ExistingFile"].is_null() {
                if parameter_type["ExistingFile"].is_string() {
                    let s = parameter_type["ExistingFile"].as_str().unwrap_or("").trim().to_lowercase();
                    if s == "lidar" {
                        file_type = ParameterFileType::Lidar;
                    } else if s == "raster" {
                        file_type = ParameterFileType::Raster;
                    } else if s == "text" {
                        file_type = ParameterFileType::Text;
                    } else if s == "html" {
                        file_type = ParameterFileType::Html;
                    } else if s == "csv" {
                        file_type = ParameterFileType::Csv;
                    } else if s == "dat" {
                        file_type = ParameterFileType::Dat;
                    } else {
                        file_type = ParameterFileType::Any;
                    }
                } else if parameter_type["ExistingFile"].is_object() {
                    let o = parameter_type["ExistingFile"].as_object().unwrap();
                    if o.contains_key("Vector") {
                        if o["Vector"].is_string() {
                            file_type = ParameterFileType::Vector;
                            let s = o["Vector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    } else if o.contains_key("RasterAndVector") {
                        if o["RasterAndVector"].is_string() {
                            file_type = ParameterFileType::RasterAndVector;
                            let s = o["RasterAndVector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    }
                }
                ParameterType::ExistingFile
            } else if !parameter_type["NewFile"].is_null() {
                if parameter_type["NewFile"].is_string() {
                    let s = parameter_type["NewFile"].as_str().unwrap_or("").trim().to_lowercase();
                    if s == "lidar" {
                        file_type = ParameterFileType::Lidar;
                    } else if s == "raster" {
                        file_type = ParameterFileType::Raster;
                    } else if s == "text" {
                        file_type = ParameterFileType::Text;
                    } else if s == "html" {
                        file_type = ParameterFileType::Html;
                    } else if s == "csv" {
                        file_type = ParameterFileType::Csv;
                    } else if s == "dat" {
                        file_type = ParameterFileType::Dat;
                    } else {
                        file_type = ParameterFileType::Any;
                    }
                } else if parameter_type["NewFile"].is_object() {
                    let o = parameter_type["NewFile"].as_object().unwrap();
                    if o.contains_key("Vector") {
                        if o["Vector"].is_string() {
                            file_type = ParameterFileType::Vector;
                            let s = o["Vector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    } else if o.contains_key("RasterAndVector") {
                        if o["RasterAndVector"].is_string() {
                            file_type = ParameterFileType::RasterAndVector;
                            let s = o["RasterAndVector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    }
                }
                ParameterType::NewFile
            } else if !parameter_type["OptionList"].is_null() {
                str_vec_value = parameter_type["OptionList"]
                .as_array()
                .unwrap_or(&empty_arr)
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_owned())
                .collect();
                if default_value.is_some() {
                    str_value = default_value.clone().unwrap();
                }
                ParameterType::OptionList
            } else if !parameter_type["FileList"].is_null() {
                if parameter_type["FileList"].is_string() {
                    let s = parameter_type["FileList"].as_str().unwrap_or("").trim().to_lowercase();
                    if s == "lidar" {
                        file_type = ParameterFileType::Lidar;
                    } else if s == "raster" {
                        file_type = ParameterFileType::Raster;
                    } else if s == "text" {
                        file_type = ParameterFileType::Text;
                    } else if s == "html" {
                        file_type = ParameterFileType::Html;
                    } else if s == "csv" {
                        file_type = ParameterFileType::Csv;
                    } else if s == "dat" {
                        file_type = ParameterFileType::Dat;
                    } else {
                        file_type = ParameterFileType::Any;
                    }
                } else if parameter_type["FileList"].is_object() {
                    let o = parameter_type["FileList"].as_object().unwrap();
                    if o.contains_key("Vector") {
                        if o["Vector"].is_string() {
                            file_type = ParameterFileType::Vector;
                            let s = o["Vector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    } else if o.contains_key("RasterAndVector") {
                        if o["RasterAndVector"].is_string() {
                            file_type = ParameterFileType::RasterAndVector;
                            let s = o["RasterAndVector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    }
                }
                ParameterType::FileList
            } else if !parameter_type["ExistingFileOrFloat"].is_null() {
                if parameter_type["ExistingFileOrFloat"].is_string() {
                    let s = parameter_type["ExistingFileOrFloat"].as_str().unwrap_or("").trim().to_lowercase();
                    if s == "lidar" {
                        file_type = ParameterFileType::Lidar;
                    } else if s == "raster" {
                        file_type = ParameterFileType::Raster;
                    } else if s == "text" {
                        file_type = ParameterFileType::Text;
                    } else if s == "html" {
                        file_type = ParameterFileType::Html;
                    } else if s == "csv" {
                        file_type = ParameterFileType::Csv;
                    } else if s == "dat" {
                        file_type = ParameterFileType::Dat;
                    } else {
                        file_type = ParameterFileType::Any;
                    }
                } else if parameter_type["ExistingFileOrFloat"].is_object() {
                    let o = parameter_type["ExistingFileOrFloat"].as_object().unwrap();
                    if o.contains_key("Vector") {
                        if o["Vector"].is_string() {
                            file_type = ParameterFileType::Vector;
                            let s = o["Vector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    } else if o.contains_key("RasterAndVector") {
                        if o["RasterAndVector"].is_string() {
                            file_type = ParameterFileType::RasterAndVector;
                            let s = o["RasterAndVector"].as_str().unwrap_or("").trim().to_lowercase();
                            if s == "any" {
                                geometry_type =VectorGeometryType::Any;
                            } else if s == "point" {
                                geometry_type =VectorGeometryType::Point;
                            } else if s == "line" {
                                geometry_type =VectorGeometryType::Line;
                            } else if s == "polygon" {
                                geometry_type =VectorGeometryType::Polygon;
                            } else if s == "lineorpolygon" {
                                geometry_type =VectorGeometryType::LineOrPolygon;
                            }
                        }
                    }
                }
                ParameterType::ExistingFileOrFloat
            } else if !parameter_type["VectorAttributeField"].is_null() {
                ParameterType::VectorAttributeField
            } else {
                println!("Object: {:?}", parameter_type);
                ParameterType::String
            }
        } else {
            println!("Something Else: {:?}", parameter_type);
            ParameterType::String
        };

        let tp = ToolParameter {
            name: name,
            flags: flags,
            description: description,
            parameter_type: pt,
            default_value: default_value,
            optional: optional,
            str_value: str_value,
            bool_value: bool_value,
            int_value: 0usize,
            // float_value: float_value,
            str_vec_value: str_vec_value,
            file_type: file_type,
            geometry_type: geometry_type,
            // attribute_type: AttributeType::Any,
        };
        ret.push(tp);
    }
    ret
}

#[derive(Default, Debug, PartialEq)]
pub enum ParameterType {
    Boolean,
    #[default]
    String,
    // StringList, // I don't think there are any tools that use this type
    Integer,
    Float,
    VectorAttributeField,
    StringOrNumber,
    ExistingFile,
    ExistingFileOrFloat,
    NewFile,
    FileList,
    Directory,
    OptionList,
}

#[derive(Default, Debug)]
pub enum ParameterFileType {
    #[default]
    Any,
    Lidar,
    Raster,
    RasterAndVector,
    Vector,
    Text,
    Html,
    Csv,
    Dat,
}


#[derive(Default, Debug)]
pub enum VectorGeometryType {
    #[default]
    Any,
    Point,
    Line,
    Polygon,
    LineOrPolygon,
}

#[derive(Default, Debug)]
pub struct ToolParameter {
    pub name: String,
    pub flags: Vec<String>,
    pub description: String,
    pub parameter_type: ParameterType,
    pub default_value: Option<String>,
    pub optional: bool,
    pub str_value: String,
    pub bool_value: bool,
    pub int_value: usize,
    // pub float_value: f32,
    pub str_vec_value: Vec<String>,
    file_type: ParameterFileType,
    geometry_type: VectorGeometryType,
}

#[derive(Default)]
pub struct ToolInfo {
    pub tool_name: String,
    pub parameters: Vec<ToolParameter>,
    json_parameters: Value,
    cancel: Arc<Mutex<bool>>,
    tool_output: Arc<Mutex<String>>,
    exe_path: String,
    working_dir: String,
    output_command: bool,
    verbose_mode: bool,
    compress_rasters: bool,
    progress: Arc<Mutex<f32>>,
    progress_label: Arc<Mutex<String>>,
    continuous_mode: Arc<Mutex<bool>>,
}

impl ToolInfo {
    pub fn new(tool_name: &str, parameters: Value) -> Self {
        let parameter_values = parse_parameters(&parameters);
        ToolInfo {
            tool_name: tool_name.to_owned(),
            parameters: parameter_values,
            json_parameters: parameters,
            cancel: Arc::new(Mutex::new(false)),
            tool_output: Arc::new(Mutex::new(String::new())),
            exe_path: String::new(),
            working_dir: String::new(),
            output_command: false,
            verbose_mode: false,
            compress_rasters: false,
            progress: Arc::new(Mutex::new(0.0)),
            progress_label: Arc::new(Mutex::new("Progress:".to_string())),
            continuous_mode: Arc::new(Mutex::new(false)),
        }
    }

    pub fn run(&mut self) {
        let mut cancel = self.cancel.lock().unwrap();
        *cancel = false;

        // self.animate_progress = true;
        if self.exe_path.trim().is_empty() {
            // we have an unspecified non-optional param
            rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Warning).set_title("Error Running Tool")
            .set_description("The WhiteboxTools executable path does not appear to be set.")
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
            return;
        }
        // Collect the parameter values
        let mut param_str = String::new(); // String::from(&format!("{} -r={} --wd={}", self.exe_path, self.tool_name, self.working_dir));
        let mut args: Vec<String> = vec![format!("-r={}", self.tool_name), format!("--wd={}", self.working_dir)];
        for parameter in &self.parameters {
            let flag = parameter.flags[parameter.flags.len()-1].clone();
            match parameter.parameter_type {
                ParameterType::Boolean => { 
                    if parameter.bool_value {
                        // param_str.push_str(&format!(" {flag}={}", parameter.bool_value));
                        param_str.push_str(&format!(" {flag}"));
                        // args.push(format!("{flag}=", parameter.bool_value));
                        args.push(format!("{flag}"));
                    } 
                },
                ParameterType::String => { 
                    if !parameter.str_value.trim().is_empty() {
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_value)); 
                        args.push(format!("{flag}='{}'", parameter.str_value));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                // ParameterType::StringList => { param_str.push_str(&format!("{flag}={:?}", parameter.str_vec_value); },
                ParameterType::Integer | ParameterType::Float => { 
                    if !parameter.str_value.trim().is_empty() {
                        if (parameter.parameter_type == ParameterType::Integer && parameter.str_value.trim().parse::<usize>().is_ok()) || 
                        (parameter.parameter_type == ParameterType::Float && parameter.str_value.trim().parse::<f32>().is_ok()){
                            param_str.push_str(&format!(" {flag}={}", parameter.str_value));
                            args.push(format!("{flag}={}", parameter.str_value));
                        } else {
                            // we had an error parsing the user intput in a number.
                            rfd::MessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                            .set_description(&format!("Error parsing a non-optional parameter {}.", parameter.name))
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                            return;
                        }
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::VectorAttributeField => { 
                    if !parameter.str_value.trim().is_empty() {
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_value)); 
                        args.push(format!("{flag}='{}'", parameter.str_value));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::StringOrNumber => {
                    if !parameter.str_value.trim().is_empty() { 
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_value)); 
                        args.push(format!("{flag}='{}'", parameter.str_value));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::ExistingFile => { 
                    if !parameter.str_value.trim().is_empty() {
                        // does the file exist?
                        if std::path::Path::new(parameter.str_value.trim()).exists() {
                            param_str.push_str(&format!(" {flag}='{}'", parameter.str_value.trim()));
                            args.push(format!("{flag}='{}'", parameter.str_value));
                        } else {
                            // maybe we just need to append the working directory...
                            if std::path::Path::new(&self.working_dir).join(&parameter.str_value.trim()).exists() {
                                param_str.push_str(&format!(" {flag}='{}'", parameter.str_value.trim()));
                                args.push(format!("{flag}='{}'", parameter.str_value));
                            } else {
                                // we have an incorrect param
                                rfd::MessageDialog::new()
                                .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                                .set_description(&format!("The specified file path does not exist. ({}).", parameter.name))
                                .set_buttons(rfd::MessageButtons::Ok)
                                .show();

                                return;
                            }
                        } 
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::ExistingFileOrFloat => {
                    if !parameter.str_value.trim().is_empty() {
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_value));
                        args.push(format!("{flag}='{}'", parameter.str_value));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::NewFile => { 
                    if !parameter.str_value.trim().is_empty() {
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_value.trim())); 
                        args.push(format!("{flag}='{}'", parameter.str_value));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::FileList => { 
                    if !parameter.str_value.trim().is_empty() {
                        let files: Vec<&str> = parameter.str_value.split("\n").collect();
                        let mut s = String::from("[");
                        for i in 0..files.len() {
                            let file = files[i].trim();
                            if !file.is_empty() && std::path::Path::new(file).exists() {
                                if i > 0 {
                                    s.push_str(&format!(",'{}'", file));
                                } else {
                                    s.push_str(&format!("'{}'", file));
                                }
                            }
                        }
                        s.push_str("]");
                        param_str.push_str(&format!(" {flag}={}", s)); 
                        args.push(format!("{flag}={}", s));
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::Directory => { 
                    if !parameter.str_value.trim().is_empty() {
                        if std::path::Path::new(parameter.str_value.trim()).exists() {
                            param_str.push_str(&format!(" {flag}='{}'", parameter.str_value));
                            args.push(format!("{flag}='{}'", parameter.str_value));
                        } else {
                            // we have an unspecified non-optional param
                            rfd::MessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                            .set_description(&format!("The specified directory does not exist. ({}).", parameter.name.trim()))
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();

                            return;
                        }  
                    } else if !parameter.optional {
                        // we have an unspecified non-optional param
                        rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                        .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();

                        return;
                    }
                },
                ParameterType::OptionList => { 
                    // if !parameter.str_value.trim().is_empty() {
                        param_str.push_str(&format!(" {flag}='{}'", parameter.str_vec_value[parameter.int_value])); 
                        args.push(format!("{flag}='{}'", parameter.str_vec_value[parameter.int_value]));
                    // } else if !parameter.optional {
                    //     // we have an unspecified non-optional param
                    //     rfd::MessageDialog::new()
                    //     .set_level(rfd::MessageLevel::Warning).set_title("Error Parsing Parameter")
                    //     .set_description(&format!("Unspecified non-optional parameter {}.", parameter.name))
                    //     .set_buttons(rfd::MessageButtons::Ok)
                    //     .show();

                    //     return;
                    // }
                },
            }
        }
        


        if self.verbose_mode {
            param_str.push_str(" -v=true");
            args.push("-v=true".to_string());
        } else {
            param_str.push_str(" -v=false");
            args.push("-v=false".to_string());
        }

        if self.compress_rasters {
            param_str.push_str(" --compress_rasters=true");
            args.push("--compress_rasters=true".to_string());
        } else {
            param_str.push_str(" --compress_rasters=false");
            args.push("--compress_rasters=false".to_string());
        }

        let continuous_mode = Arc::clone(&self.continuous_mode);
        let mut cm = continuous_mode.lock().unwrap();
        *cm = true;

        let tool_output = Arc::clone(&self.tool_output);
        let mut to = tool_output.lock().unwrap();

        // to.push_str(&format!("Running {}...\n", self.tool_name));
        // // to.push_str(&param_str);

        if self.output_command {
            to.push_str(
                &format!(
                    "{} -r={} --wd=\"{}\" {}\n", 
                    &self.exe_path, 
                    self.tool_name, 
                    self.working_dir, 
                    param_str
                )
            );
        }

        // // this works with stdout but not with the gui.
        // let cmd = duct::cmd(&self.exe_path, &args);
        // let reader = cmd.stderr_to_stdout().reader().unwrap();
        // let mut lines = BufReader::new(reader).lines();
        // while let Some(line) = lines.next() {
        //     // match line.unwrap() {
        //     //     Some(line) => {
        //     //         println!("{line}");
        //     //         self.tool_output.push_str(&format!("{}\n",line));
        //     //     },
        //     //     None => {
        //     //         println!("Error")
        //     //         self.tool_output.push_str("Error");
        //     //         break;
        //     //     }
        //     // }
        //     println!("{:?}", line);
        //     self.tool_output.push_str(&format!("{:?}\n", line));
        // }


        // let output = Command::new(&self.exe_path)
        //     .args(&args)
        //     .stdout(Stdio::piped())
        //     .output().unwrap();

        // println!("status: {}", output.status);
        // std::io::stdout().write_all(&output.stdout).unwrap();
        // std::io::stdout().write_all(&output.stderr).unwrap();

        let exe_path = Arc::new(self.exe_path.clone());
        let exe = Arc::clone(&exe_path);
        let pcnt = Arc::clone(&self.progress);
        let progress_label = Arc::clone(&self.progress_label);
        let continuous_mode = Arc::clone(&self.continuous_mode);
        let tool_output = Arc::clone(&self.tool_output);
        let cancel = Arc::clone(&self.cancel);
        thread::spawn(move || {
            let mut child = Command::new(&*exe)
                .args(&args)
                .stdout(Stdio::piped())
                .spawn().unwrap();

            let mut stdout = child.stdout.take().unwrap();

            let mut buf = [0u8; 200];
            let mut do_read = || -> usize {
                let read = stdout.read(&mut buf).unwrap();
                let line = std::str::from_utf8(&buf[0..read]).unwrap();
                let mut to = tool_output.lock().unwrap();

                if line.contains("%") {
                    let val1: Vec<&str> = line.split(":").collect::<Vec<&str>>();
                    let percent_val = val1[1].replace("%", "").trim().parse::<f32>().unwrap_or(0.0);
                    // println!("{percent_val}");
                    let mut val = pcnt.lock().unwrap();
                    *val = percent_val / 100.0;
                    let mut val2 = progress_label.lock().unwrap();
                    *val2 = val1[0].to_string();
                }
                to.push_str(&format!("{line}"));

                // let str_data = std::str::from_utf8(&buf[0..read]).unwrap();
                // let lines = str_data.split("\n").collect::<Vec<&str>>();
                // for line in &lines {
                //     if line.contains("%") {
                //         let val1: Vec<&str> = line.split(":").collect::<Vec<&str>>();
                //         let percent_val = val1[1].replace("%", "").trim().parse::<f32>().unwrap_or(0.0);
                //         // println!("{percent_val}");
                //         let mut val = pcnt.lock().unwrap();
                //         *val = percent_val / 100.0;
                //         let mut val2 = progress_label.lock().unwrap();
                //         *val2 = val1[0].to_string();
                //     } else {
                //         to.push_str(&format!("{line}\n"));
                //     }
                // }
                
                std::io::stdout().flush().unwrap();
                read
            };

            let mut last;
            while child.try_wait().unwrap().is_none() {
                let mut cancel2 = cancel.lock().unwrap();
                if *cancel2 {
                    // cancel the process
                    let mut to = tool_output.lock().unwrap();
                    to.push_str("\nCancelling process...\n");

                    child.kill().expect("Error encountered while killing process");
                    *cancel2 = false; // reset the cancel.

                    to.push_str("\nProcess cancelled\n");
                }

                let _last = do_read();
            }

            // println!("{}", child.try_wait().unwrap().unwrap());

            // make sure we try at least one more read in case there's new data in the pipe after the child exited
            last = 1;

            while last > 0 {
                last = do_read();
            }

            let mut val = pcnt.lock().unwrap();
            *val = 0.0;

            let mut val2 = progress_label.lock().unwrap();
            *val2 = "Progress".to_string();

            let mut cm = continuous_mode.lock().unwrap();
            *cm = false;
        });

        // self.animate_progress = false;
    }

    pub fn cancel(&mut self) {
        // self.cancel = true;
        // println!("Cancelling {} ({})", self.tool_name, self.cancel);
        let mut cancel = self.cancel.lock().unwrap();
        *cancel = true;
    }

    pub fn reset(&mut self) {
        self.parameters = parse_parameters(&self.json_parameters);
        let mut cancel = self.cancel.lock().unwrap();
        *cancel = false;
        let mut tool_output = self.tool_output.lock().unwrap();
        *tool_output = String::new();
        
        let mut val = self.progress.lock().unwrap();
        *val = 0.0;

        let mut val2 = self.progress_label.lock().unwrap();
        *val2 = "Progress".to_string();
    }

    pub fn update_exe_path(&mut self, exe_path: &str) {
        self.exe_path = exe_path.to_string();
    }

    pub fn update_working_dir(&mut self, working_dir: &str) {
        self.working_dir = working_dir.to_string();
    }

    pub fn update_output_command(&mut self, value: bool) {
        self.output_command = value;
    }

    pub fn update_verbose_mode(&mut self, value: bool) {
        self.verbose_mode = value;
    }

    pub fn update_compress_rasters(&mut self, value: bool) {
        self.compress_rasters = value;
    }

    fn get_tool_help(&self) -> Option<String> {
        let output = Command::new(&self.exe_path)
                .args([format!("--toolhelp={}", self.tool_name)])
                .output()
                .expect("Could not execute the WhiteboxTools binary");
        
        if output.status.success() {
            let s = match std::str::from_utf8(&(output.stdout)) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            return Some(s.to_string());
        } else {
            panic!("Could not execute the WhiteboxTools binary");
        }
    }
}

impl MyApp {

    pub fn tool_dialog(&mut self, ctx: &egui::Context, i: usize) {
        let mut close_dialog = false;
        self.get_tool_parameters(&self.tool_info[i].tool_name);
        egui::Window::new(&format!("{}", &self.tool_info[i].tool_name))
        .open(&mut self.open_tools[i])
        .resizable(true)
        .vscroll(true)
        .show(ctx, |ui| {
            egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .striped(true)
            .show(ui, |ui| {

                ui.label("Tool Parameters:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟲").on_hover_text("Reset parameters").clicked() {
                        self.tool_info[i].reset();
                    }
                });
                ui.end_row();
            

                for parameter in &mut (self.tool_info[i].parameters) {
                    let suffix = if parameter.optional { "*".to_string() } else { "".to_string() };
                    let parameter_label = if parameter.name.len() + suffix.len() < 25 {
                        format!("{}{}", &parameter.name, suffix)
                    } else {
                        format!("{}...{}", &parameter.name[0..(22-suffix.len())], suffix)
                    };
                    let param_nm = if !parameter.optional { parameter.name.clone() } else { format!("{} [Optional]", parameter.name) };
                    let hover_text = match parameter.file_type {
                        ParameterFileType::Vector | ParameterFileType::RasterAndVector => {
                            format!("{}:  {} (Geometry Type={:?})", param_nm, parameter.description, parameter.geometry_type)
                        },
                        _ => {
                            format!("{}:  {}", param_nm, parameter.description)
                        }
                    };
                    ui.label(&parameter_label)
                    .on_hover_text(&hover_text);

                    match parameter.parameter_type {
                        ParameterType::Boolean => {
                            ui.add(toggle(&mut parameter.bool_value));
                        },
                        ParameterType::Directory => {
                            if ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            ).double_clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(mut path) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_file() {
                                    parameter.str_value = path.display().to_string();
                                    // update the working directory
                                    path.pop();
                                    self.state.working_dir = path.display().to_string();
                                }
                            }
                            if ui.button("…").clicked() {
                                if let Some(path) = rfd::FileDialog::new().set_directory(std::path::Path::new(&self.state.working_dir)).pick_folder() {
                                    parameter.str_value = path.display().to_string();
                                }
                            }
                        },
                        ParameterType::ExistingFile => {
                            if ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            ).double_clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(mut path) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_file() {
                                    parameter.str_value = path.display().to_string();
                                    // update the working directory
                                    path.pop();
                                    self.state.working_dir = path.display().to_string();
                                }
                            }

                            if ui.button("…").clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(mut path) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_file() {
                                    parameter.str_value = path.display().to_string();
                                    // update the working directory
                                    path.pop();
                                    self.state.working_dir = path.display().to_string();
                                }
                            }
                        },
                        ParameterType::ExistingFileOrFloat => {
                            ui.horizontal(|ui| {
                                if ui.add(
                                    egui::TextEdit::singleline(&mut parameter.str_value)
                                    .desired_width(self.state.textbox_width)
                                ).double_clicked() {
                                    let fdialog = get_file_dialog(&parameter.file_type); 
                                    if let Some(mut path) = fdialog
                                    .set_directory(std::path::Path::new(&self.state.working_dir))
                                    .pick_file() {
                                        parameter.str_value = path.display().to_string();
                                        // update the working directory
                                        path.pop();
                                        self.state.working_dir = path.display().to_string();
                                    }
                                }
                                if ui.button("…").clicked() {
                                    let fdialog = get_file_dialog(&parameter.file_type); 
                                    if let Some(mut path) = fdialog
                                    .set_directory(std::path::Path::new(&self.state.working_dir))
                                    .pick_file() {
                                        parameter.str_value = path.display().to_string();
                                        // update the working directory
                                        path.pop();
                                        self.state.working_dir = path.display().to_string();
                                    }
                                }

                                ui.label("OR");
                                
                                ui.add(
                                    egui::TextEdit::singleline(&mut parameter.str_value)
                                    .desired_width(50.0)
                                );
                            });
                        },
                        ParameterType::FileList => {
                            if ui.add(
                                egui::TextEdit::multiline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            ).double_clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(mut path) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_file() {
                                    parameter.str_value = path.display().to_string();
                                    // update the working directory
                                    path.pop();
                                    self.state.working_dir = path.display().to_string();
                                }
                            }
                            if ui.button("…").clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type);

                                if let Some(mut paths) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_files() {
                                    // let s = String::new();
                                    for path in &paths {
                                        parameter.str_value.push_str(&format!("{}\n", path.display().to_string()));
                                    }
                                    
                                    // update the working directory
                                    paths[0].pop();
                                    self.state.working_dir = paths[0].display().to_string();
                                }
                            }
                        }
                        ParameterType::Float | ParameterType::Integer => {
                            // ui.add(egui::DragValue::new(&mut parameter.float_value).speed(0).max_decimals(5));
                            ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(50.0) //self.state.textbox_width)
                            );

                            // let text_edit = egui::TextEdit::singleline(&mut parameter.str_value)
                            // .desired_width(50.0);
                            // let output = text_edit.show(ui);
                            // if output.response.double_clicked() {
                            //     // What to do here?
                            // }

                        },
                        ParameterType::NewFile => {
                            // ui.text_edit_singleline(&mut parameter.str_value);
                            if ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            ).double_clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(mut path) = fdialog
                                .set_directory(std::path::Path::new(&self.state.working_dir))
                                .pick_file() {
                                    parameter.str_value = path.display().to_string();
                                    // update the working directory
                                    path.pop();
                                    self.state.working_dir = path.display().to_string();
                                }
                            }
                            if ui.button("…").clicked() {
                                let fdialog = get_file_dialog(&parameter.file_type); 
                                if let Some(path) = fdialog.set_directory(std::path::Path::new(&self.state.working_dir)).save_file() {
                                    parameter.str_value = path.display().to_string();
                                }
                            }
                        },
                        ParameterType::OptionList => {
                            let alternatives = parameter.str_vec_value.clone();
                            egui::ComboBox::from_id_source(&parameter.name).show_index(
                                ui,
                                &mut parameter.int_value,
                                alternatives.len(),
                                |i| alternatives[i].to_owned()
                            );
                        }
                        ParameterType::String => {
                            ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            );
                        },
                        ParameterType::StringOrNumber => {
                            ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            );
                        },
                        ParameterType::VectorAttributeField => {
                            ui.add(
                                egui::TextEdit::singleline(&mut parameter.str_value)
                                .desired_width(self.state.textbox_width)
                            );
                        },
                    }
                    
                    ui.end_row();
                }
            });

            ui.separator();
            ui.vertical(|ui| {
                ui.set_height(170.);
                ui.horizontal(|ui| {
                    ui.label("Tool Output:");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").on_hover_text("Clear tool output").clicked() {
                            let mut tool_output = self.tool_info[i].tool_output.lock().unwrap();
                            *tool_output = "".to_string();
                        }
                    });
                });
                // ui.label("Tool Output:");

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut tool_output = self.tool_info[i].tool_output.lock().unwrap();
                    ui.add(
                        egui::TextEdit::multiline(&mut *tool_output)
                            .cursor_at_end(true)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                    );
                    let cm = self.tool_info[i].continuous_mode.lock().unwrap();
                    if *cm {
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    self.tool_info[i].update_working_dir(&self.state.working_dir);
                    self.tool_info[i].run();
                    // let runner = Arc::new(self.tool_info[i]);
                    // std::thread::spawn(move || {
                    //     self.tool_info[i].run();
                    // });
                }
                if ui.button("Cancel").clicked() {
                    self.tool_info[i].cancel();
                }
                if ui.button("Help").clicked() {
                    // println!("Help for {}...", self.tool_info[i].tool_name);
                    let help_str = self.tool_info[i].get_tool_help();
                    if help_str.is_some() {
                        // println!("{}", help_str.unwrap());
                        let mut tool_output = self.tool_info[i].tool_output.lock().unwrap();
                        *tool_output = help_str.unwrap_or("".to_string());
                    }
                }
                if ui.button("Close").clicked() {
                    close_dialog = true;
                }
                let progress = *(self.tool_info[i].progress).lock().unwrap();
                let progress_label = &*(self.tool_info[i].progress_label).lock().unwrap();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::ProgressBar::new(progress)
                    .desired_width(100.0)
                    .show_percentage());

                    ui.label(progress_label);
                })
            });

            let cm = self.tool_info[i].continuous_mode.lock().unwrap();
            if *cm {
                ctx.request_repaint();
            }
        });

        if close_dialog {
            self.open_tools[i] = false;
        }
    }
}


fn get_file_dialog(pft: &ParameterFileType) -> rfd::FileDialog {
    match pft {
        ParameterFileType::Lidar => {
            rfd::FileDialog::new()
            .add_filter("LAS Files", &["las"])
            .add_filter("LAZ Files", &["laz"])
            .add_filter("zLidar Files", &["zLidar"])
            .add_filter("Lidar Files", &["las", "laz", "zLidar"])
        },
        ParameterFileType::Raster => {
            rfd::FileDialog::new()
            .add_filter("Raster Files", &["tif", "tiff", "bil", "hdr", "flt", "sdat", "sgrd", "rdc", "rst", "grd", "txt", "asc", "tas", "dep"])
            .add_filter("GeoTIFF Files", &["tif", "tiff"])
        },
        ParameterFileType::Vector => {
            rfd::FileDialog::new()
            .add_filter("Vector Files", &["shp"])
        },
        ParameterFileType::RasterAndVector => {
            rfd::FileDialog::new()
            .add_filter("Raster Files", &["tif", "tiff", "bil", "hdr", "flt", "sdat", "sgrd", "rdc", "rst", "grd", "txt", "asc", "tas", "dep"])
            .add_filter("Vector Files", &["shp"])
        },
        ParameterFileType::Text => {
            rfd::FileDialog::new()
            .add_filter("Test Files", &["txt"])
        },
        ParameterFileType::Html => {
            rfd::FileDialog::new()
            .add_filter("HTML Files", &["html"])
        },
        ParameterFileType::Csv => {
            rfd::FileDialog::new()
            .add_filter("CSV Files", &["csv"])
        },
        ParameterFileType::Dat => {
            rfd::FileDialog::new()
            .add_filter("DAT Files", &["dat"])
        },
        _ => { 
            rfd::FileDialog::new()
        }
    }
}

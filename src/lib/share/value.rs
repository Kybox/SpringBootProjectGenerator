
// Packaging accepted
pub const JAR: &str = "jar";
pub const WAR: &str = "war";

// Java versions accepted
pub const JAVA_8: &str = "1.8";
pub const JAVA_11: &str = "11";
pub const JAVA_12: &str = "12";

pub const JAVA_PACKAGE: &str = "src\\main\\java";
pub const SOAP_WS_CONFIG_CLASS: &str = "config\\WebServiceConfig.java";

// Yes or No response
pub const YES: &str = "y";
pub const NO: &str= "n";

// BASIC INFOS MESSAGES
pub const INFO_GROUP_ID: &str= "Enter the groupId :";
pub const INFO_ARTIFACT_ID: &str= "Enter the artifactId :";
pub const INFO_PROJECT_VERSION: &str= "Enter the project version :";
pub const INFO_PROJECT_NAME: &str= "Enter the name of the project :";
pub const INFO_PROJECT_DESCRIPTION: &str= "Enter the project description (optional) :";
pub const INFO_PACKAGING: &str= "Enter the packaging (jar or war) :";
pub const INFO_JAVA_VERSION: &str= "Enter the Java version (1.8, 11 or 12) :";

// TEMPLATE SELECTION
pub const INFO_TEMPLATE_SELECTION: &str= "Select a project type ";
pub const SIMPLE_PROJECT: &str= "Simple project";
pub const RESTFULL_WEBSERVICE: &str= "RESTful Webservice";
pub const SOAP_WEBSERVICE: &str= "SOAP Webservice";

// TEMPLATE FILES
pub const TMP_POM_FILE: &str = "pom.tmp";
pub const H2_TEMP_FILE: &str = "dependency/h2-db.tmp";
pub const DATA_REST_TEMP_FILE: &str = "dependency/spring-data-rest.tmp";
pub const DATA_JPA_TEMP_FILE: &str = "dependency/spring-data-jpa.tmp";
pub const WS_TEMP_FILE: &str = "dependency/spring-ws.tmp";
pub const MAVEN_JAXB2_PLUGIN_TEMP_FILE: &str = "plugin/maven-jaxb2-plugin.tmp";
pub const MAIN_CLASS_TEMP_FILE: &str = "class/main.tmp";
pub const WS_CONFIG_CLASS_TEMP_FILE: &str = "class/spring-ws-config.tmp";

// HASHTAGS
pub const HASHTAG_GROUP_ID: &str = "#groupId";
pub const HASHTAG_ARTIFACT_ID: &str = "#artifactId";
pub const HASHTAG_VERSION: &str = "#version";
pub const HASHTAG_JAVA: &str = "#java";
pub const HASHTAG_NAME: &str = "#name";
pub const HASHTAG_DESC: &str = "#desc";
pub const HASHTAG_PACKAGING: &str = "#packaging";
pub const HASHTAG_H2DB: &str = "#h2db";
pub const HASHTAG_DATA_REST: &str = "#spring-data-rest";
pub const HASHTAG_DATA_JPA: &str = "#spring-data-jpa";
pub const HASHTAG_WS: &str = "#spring-ws";
pub const HASHTAG_MAVEN_JAXB2_PLUGIN: &str = "#maven-jaxb2-plugin";

// ERROR MESSAGES
pub const ERROR_GROUP_ID_1_1: &str= "It seems that the groupId is malformed...";
pub const ERROR_GROUP_ID_1_2: &str= "Please, grap some words like com.domain.projet";
pub const ERROR_ARTIFACT_ID_1_1: &str= "It seems that the artifactId is malformed...";
pub const ERROR_ARTIFACT_ID_1_2: &str= "Please, grap some words like my-awesome-project";
pub const ERROR_PROJECT_VERSION_1_1: &str= "The project version should not be empty...";
pub const ERROR_PROJECT_VERSION_1_2: &str= "-> Grab a version like 0.1-SNAPSHOT";
pub const ERROR_PACKAGING_1_1: &str= "Wrong packaging...";
pub const ERROR_PACKAGING_1_2: &str= "-> Grab jar or war";
pub const ERROR_JAVA_VERSION_1_1: &str= "Java version unsupported...";
pub const ERROR_JAVA_VERSION_1_2: &str= "-> Grab 1.8, 11 or 12";

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET All To-Dos</name>
   <tag></tag>
   <elementGuidId>eafe1a69-fc99-4449-94d4-a65a2bef9c9a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/todos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[26].userId', '2')
WS.verifyElementPropertyValue(response, '[26].id', '27')
WS.verifyElementPropertyValue(response, '[26].title', 'veritatis pariatur delectus')
WS.verifyElementPropertyValue(response, '[26].completed', 'true')


WS.verifyElementPropertyValue(response, '[27].userId', '2')
WS.verifyElementPropertyValue(response, '[27].id', '28')
WS.verifyElementPropertyValue(response, '[27].title', 'nesciunt totam sit blanditiis sit')
WS.verifyElementPropertyValue(response, '[27].completed', 'false')

WS.verifyElementPropertyValue(response, '[29].userId', '2')
WS.verifyElementPropertyValue(response, '[29].id', '30')
WS.verifyElementPropertyValue(response, '[29].title', 'nemo perspiciatis repellat ut dolor libero commodi blanditiis omnis')
WS.verifyElementPropertyValue(response, '[29].completed', 'true')

WS.verifyElementPropertyValue(response, '[62].userId', '4')
WS.verifyElementPropertyValue(response, '[62].id', '63')
WS.verifyElementPropertyValue(response, '[62].title', 'doloremque aut dolores quidem fuga qui nulla')
WS.verifyElementPropertyValue(response, '[62].completed', 'true')

WS.verifyElementPropertyValue(response, '[63].userId', '4')
WS.verifyElementPropertyValue(response, '[63].id', '64')
WS.verifyElementPropertyValue(response, '[63].title', 'voluptas consequatur qui ut quia magnam nemo esse')
WS.verifyElementPropertyValue(response, '[63].completed', 'false')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

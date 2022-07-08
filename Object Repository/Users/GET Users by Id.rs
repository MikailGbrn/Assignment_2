<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Users by Id</name>
   <tag></tag>
   <elementGuidId>6103e531-0ab6-4d74-afa1-fbb10324af0b</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/users/1</restUrl>
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
WS.verifyElementPropertyValue(response, 'id', '1')
WS.verifyElementPropertyValue(response, 'name', 'Leanne Graham')
WS.verifyElementPropertyValue(response, 'username', 'Bret')
WS.verifyElementPropertyValue(response, 'email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, 'address.street', 'Kulas Light')
WS.verifyElementPropertyValue(response, 'address.suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, 'address.city', 'Gwenborough')
WS.verifyElementPropertyValue(response, 'address.zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'phone', '1-770-736-8031 x56442')
WS.verifyElementPropertyValue(response, 'website', 'hildegard.org')
WS.verifyElementPropertyValue(response, 'company.name', 'Romaguera-Crona')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, 'company.bs', 'harness real-time e-markets')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

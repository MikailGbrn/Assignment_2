<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET All Albums</name>
   <tag></tag>
   <elementGuidId>6646c538-7efc-4370-8c3f-894ec3d515b2</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/albums</restUrl>
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
WS.verifyElementPropertyValue(response, '[19].userId', '2')
WS.verifyElementPropertyValue(response, '[19].id', '20')
WS.verifyElementPropertyValue(response, '[19].title', 'voluptas rerum iure ut enim')

WS.verifyElementPropertyValue(response, '[28].userId', '3')
WS.verifyElementPropertyValue(response, '[28].id', '29')
WS.verifyElementPropertyValue(response, '[28].title', 'inventore ut quasi magnam itaque est fugit')

WS.verifyElementPropertyValue(response, '[30].userId', '4')
WS.verifyElementPropertyValue(response, '[30].id', '31')
WS.verifyElementPropertyValue(response, '[30].title', 'adipisci laborum fuga laboriosam')

WS.verifyElementPropertyValue(response, '[31].userId', '4')
WS.verifyElementPropertyValue(response, '[31].id', '32')
WS.verifyElementPropertyValue(response, '[31].title', 'reiciendis dolores a ut qui debitis non quo labore')

WS.verifyElementPropertyValue(response, '[52].userId', '6')
WS.verifyElementPropertyValue(response, '[53].id', '54')
WS.verifyElementPropertyValue(response, '[53].title', 'aut non illo amet perferendis')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

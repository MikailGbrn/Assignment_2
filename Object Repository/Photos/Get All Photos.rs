<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get All Photos</name>
   <tag></tag>
   <elementGuidId>a794b83a-e099-45b3-87ac-65e8488fd41b</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/photos</restUrl>
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

WS.verifyElementPropertyValue(response, '[0].albumId', '1')
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].title', 'accusamus beatae ad facilis cum similique qui sunt')
WS.verifyElementPropertyValue(response, '[0].url', 'https://via.placeholder.com/600/92c952')
WS.verifyElementPropertyValue(response, '[0].thumbnailUrl', 'https://via.placeholder.com/150/92c952')

WS.verifyElementPropertyValue(response, '[21].albumId', '1')
WS.verifyElementPropertyValue(response, '[21].id', '22')
WS.verifyElementPropertyValue(response, '[21].title', 'et ea illo et sit voluptas animi blanditiis porro')
WS.verifyElementPropertyValue(response, '[21].url', 'https://via.placeholder.com/600/45601a')
WS.verifyElementPropertyValue(response, '[21].thumbnailUrl', 'https://via.placeholder.com/150/45601a')

WS.verifyElementPropertyValue(response, '[13].albumId', '1')
WS.verifyElementPropertyValue(response, '[13].id', '14')
WS.verifyElementPropertyValue(response, '[13].title', 'est necessitatibus architecto ut laborum')
WS.verifyElementPropertyValue(response, '[13].url', 'https://via.placeholder.com/600/61a65')
WS.verifyElementPropertyValue(response, '[13].thumbnailUrl', 'https://via.placeholder.com/150/61a65')

WS.verifyElementPropertyValue(response, '[39].albumId', '1')
WS.verifyElementPropertyValue(response, '[39].id', '40')
WS.verifyElementPropertyValue(response, '[39].title', 'est quas voluptates dignissimos sint praesentium nisi recusandae')
WS.verifyElementPropertyValue(response, '[39].url', 'https://via.placeholder.com/600/3a0b95')
WS.verifyElementPropertyValue(response, '[39].thumbnailUrl', 'https://via.placeholder.com/150/3a0b95')

WS.verifyElementPropertyValue(response, '[55].albumId', '2')
WS.verifyElementPropertyValue(response, '[55].id', '56')
WS.verifyElementPropertyValue(response, '[55].title', 'vel voluptatem esse consequuntur est officia quo aut quisquam')
WS.verifyElementPropertyValue(response, '[55].url', 'https://via.placeholder.com/600/f9f067')
WS.verifyElementPropertyValue(response, '[55].thumbnailUrl', 'https://via.placeholder.com/150/f9f067')

WS.verifyElementPropertyValue(response, '[84].albumId', '2')
WS.verifyElementPropertyValue(response, '[84].id', '85')
WS.verifyElementPropertyValue(response, '[84].title', 'ullam delectus architecto sint error')
WS.verifyElementPropertyValue(response, '[84].url', 'https://via.placeholder.com/600/eb7e7f')
WS.verifyElementPropertyValue(response, '[84].thumbnailUrl', 'https://via.placeholder.com/150/eb7e7f')

WS.verifyElementPropertyValue(response, '[90].albumId', '2')
WS.verifyElementPropertyValue(response, '[90].id', '91')
WS.verifyElementPropertyValue(response, '[90].title', 'sunt quo laborum commodi porro consequatur nam delectus et')
WS.verifyElementPropertyValue(response, '[90].url', 'https://via.placeholder.com/600/40591')
WS.verifyElementPropertyValue(response, '[90].thumbnailUrl', 'https://via.placeholder.com/150/40591')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

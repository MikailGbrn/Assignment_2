<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET All Comments</name>
   <tag></tag>
   <elementGuidId>8f9efe70-fbfc-433a-97dc-429ebe1b01a9</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/comments</restUrl>
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

WS.verifyElementPropertyValue(response, '[10].postId', '3')
WS.verifyElementPropertyValue(response, '[10].id', '11')
WS.verifyElementPropertyValue(response, '[10].name', 'fugit labore quia mollitia quas deserunt nostrum sunt')
WS.verifyElementPropertyValue(response, '[10].email', 'Veronica_Goodwin@timmothy.net')
WS.verifyElementPropertyValue(response, '[10].body', 'ut dolorum nostrum id quia aut est\nfuga est inventore vel eligendi explicabo quis consectetur\naut occaecati repellat id natus quo est\nut blanditiis quia ut vel ut maiores ea')

WS.verifyElementPropertyValue(response, '[19].postId', '4')
WS.verifyElementPropertyValue(response, '[19].id', '20')
WS.verifyElementPropertyValue(response, '[19].name', 'molestias expedita iste aliquid voluptates')
WS.verifyElementPropertyValue(response, '[19].email', 'Mariana_Orn@preston.org')
WS.verifyElementPropertyValue(response, '[19].body', 'qui harum consequatur fugiat\net eligendi perferendis at molestiae commodi ducimus\ndoloremque asperiores numquam qui\nut sit dignissimos reprehenderit tempore')


WS.verifyElementPropertyValue(response, '[13].postId', '3')
WS.verifyElementPropertyValue(response, '[13].id', '14')
WS.verifyElementPropertyValue(response, '[13].name', 'et officiis id praesentium hic aut ipsa dolorem repudiandae')
WS.verifyElementPropertyValue(response, '[13].email', 'Nathan@solon.io')
WS.verifyElementPropertyValue(response, '[13].body', 'vel quae voluptas qui exercitationem\nvoluptatibus unde sed\nminima et qui ipsam aspernatur\nexpedita magnam laudantium et et quaerat ut qui dolorum')

WS.verifyElementPropertyValue(response, '[31].postId', '7')
WS.verifyElementPropertyValue(response, '[31].id', '32')
WS.verifyElementPropertyValue(response, '[31].name', 'dolorem architecto ut pariatur quae qui suscipit')
WS.verifyElementPropertyValue(response, '[31].email', 'Maria@laurel.name')
WS.verifyElementPropertyValue(response, '[31].body', 'nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus')


WS.verifyElementPropertyValue(response, '[466].postId', '94')
WS.verifyElementPropertyValue(response, '[466].id', '467')
WS.verifyElementPropertyValue(response, '[466].name', 'voluptas unde perferendis ut eaque dicta')
WS.verifyElementPropertyValue(response, '[466].email', 'Adrianna_Howell@molly.io')
WS.verifyElementPropertyValue(response, '[466].body', 'deleniti fuga hic autem\nsed rerum non voluptate sit totam consequuntur illo\nquasi quod aut ducimus dolore distinctio molestias\nnon velit quis debitis cumque voluptas')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

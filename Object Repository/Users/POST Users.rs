<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Users</name>
   <tag></tag>
   <elementGuidId>ca02d66b-f01b-4289-ada3-eea1c42ec31a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;Mikail Gibran\&quot;,\n  \&quot;username\&quot;: \&quot;Gbrn\&quot;,\n  \&quot;email\&quot;: \&quot;Space.Bvgati@BFI.biz\&quot;,\n  \&quot;address\&quot;: {\n    \&quot;street\&quot;: \&quot;Jl. Mawar 12\&quot;,\n    \&quot;suite\&quot;: \&quot;House\&quot;,\n    \&quot;city\&quot;: \&quot;Tangerang\&quot;,\n    \&quot;zipcode\&quot;: \&quot;15810\&quot;,\n    \&quot;geo\&quot;: {\n      \&quot;lat\&quot;: \&quot;-37.09876\&quot;,\n      \&quot;lng\&quot;: \&quot;-0098.123321\&quot;\n    }\n  },\n  \&quot;phone\&quot;: \&quot;+628-59-2996-7251\&quot;,\n  \&quot;website\&quot;: \&quot;behance.net/geebs\&quot;,\n  \&quot;company\&quot;: {\n    \&quot;name\&quot;: \&quot;BFI Indonesia\&quot;,\n    \&quot;catchPhrase\&quot;: \&quot;Anda Sopan, Kami Segan!!\&quot;,\n    \&quot;bs\&quot;: \&quot;haha ini apa ya\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>31ea189e-2b49-40a8-aee7-8dfe29642d9b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
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


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

assertThat(response.getResponseText()).contains('Mikail Gibran')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>list_insight</name>
   <tag></tag>
   <elementGuidId>92f91ff4-23e8-46c2-9458-06c96fb141a0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJFUzI1NiIsImtpZCI6IjAxSFdZTjRLWTc0UzZETjNWOUgxOEcxUVFSXzE3MTQ3MjA3NTUiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOiJhdXRoIiwiZXhwIjoxNzQ1NDgwMjIzLCJqdGkiOiIwMUpTS0FTS1I0MTZOWk1FNjAzMloyVzY0QiIsImlhdCI6MTc0NTQ3OTMyMywiaXNzIjoiaHR0cHM6Ly9hdXRoLmdodGtsYWIuY29tIiwic3ViIjoiMDFKSDdSU1pEWEI0OEhYWjg3WDkzR1hYNDQiLCJzY3AiOlsib2ZmbGluZV9hY2Nlc3MiLCJvcGVuaWQiXSwic2lkIjoibk1pMUhMQ0xXMUpBTXhCdjRQZTVWbjlSaFFBMkNicFQiLCJjbGllbnRfaWQiOiIwMUhXWU40S1k3NFM2RE4zVjlIMThHMVFRUiIsInR5cGUiOiJvYXV0aCJ9.Yn-c_xI1m-vFBG9LyEwBoI69oTN-3u8xdktD4ACaoDUxDaq56ctcJi7MpT0X8bBnlPOJ1D5Vqt0kFEXNO2ESUw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;page\&quot;: 1,\n    \&quot;limit\&quot;: 10,\n    \&quot;filters\&quot;: [\n        {\n            \&quot;filter_type\&quot;: \&quot;RIS_NAME\&quot;,\n            \&quot;values\&quot;: \&quot;kinh doanh\&quot;\n               \n        \n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>548c4221-cb01-4c9a-9805-d8ae9113aa29</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJFUzI1NiIsImtpZCI6IjAxSFdZTjRLWTc0UzZETjNWOUgxOEcxUVFSXzE3MTQ3MjA3NTUiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOiJhdXRoIiwiZXhwIjoxNzQ1NDgwMjIzLCJqdGkiOiIwMUpTS0FTS1I0MTZOWk1FNjAzMloyVzY0QiIsImlhdCI6MTc0NTQ3OTMyMywiaXNzIjoiaHR0cHM6Ly9hdXRoLmdodGtsYWIuY29tIiwic3ViIjoiMDFKSDdSU1pEWEI0OEhYWjg3WDkzR1hYNDQiLCJzY3AiOlsib2ZmbGluZV9hY2Nlc3MiLCJvcGVuaWQiXSwic2lkIjoibk1pMUhMQ0xXMUpBTXhCdjRQZTVWbjlSaFFBMkNicFQiLCJjbGllbnRfaWQiOiIwMUhXWU40S1k3NFM2RE4zVjlIMThHMVFRUiIsInR5cGUiOiJvYXV0aCJ9.Yn-c_xI1m-vFBG9LyEwBoI69oTN-3u8xdktD4ACaoDUxDaq56ctcJi7MpT0X8bBnlPOJ1D5Vqt0kFEXNO2ESUw</value>
      <webElementGuid>b2d0dc4d-6435-4d2d-a220-ebbe1f2f2f6b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://das.ghtklab.com/api/gi-ruleinsight/view</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

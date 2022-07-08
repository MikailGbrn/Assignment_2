import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Photos/Get All Photos'))

WS.verifyResponseStatusCode(response, 200)

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
WS.verifyElementPropertyValue(response, '[90].thumbnailUrl', 'https://via.placeholder.com/150/40591')


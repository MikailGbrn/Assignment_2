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

response = WS.sendRequest(findTestObject('Comments/Get Comments by Id'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'postId', '16')
WS.verifyElementPropertyValue(response, 'id', '78')
WS.verifyElementPropertyValue(response, 'name', 'voluptas voluptas voluptatibus blanditiis')
WS.verifyElementPropertyValue(response, 'email', 'Catharine@jordyn.com')
WS.verifyElementPropertyValue(response, 'body', 'qui adipisci eveniet excepturi iusto magni et\nenim ducimus asperiores blanditiis nemo\ncommodi nihil ex\nenim rerum vel nobis nostrum et non')